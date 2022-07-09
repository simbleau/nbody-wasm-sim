use std::collections::HashMap;

use gloo_console::log;
use wgpu::{include_wgsl, ShaderModule};
use winit::window::Window;

use crate::{renderer, state::State};

pub struct WgpuContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub shaders: HashMap<&'static str, ShaderModule>,
}

impl WgpuContext {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        log!("Surface size:", size.width, size.height);

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let backend = format!("{:?}", adapter.get_info().backend);
        log!("Backend:", backend);

        std::mem::drop(instance);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we disable most features.
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
        };
        surface.configure(&device, &config);

        let vert_shader =
            device.create_shader_module(include_wgsl!("shaders/vert.wgsl"));
        let frag_shader =
            device.create_shader_module(include_wgsl!("shaders/frag.wgsl"));
        let mut shaders = HashMap::new();
        shaders.insert("vert", vert_shader);
        shaders.insert("frag", frag_shader);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            shaders,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self, state: &State) -> Result<(), wgpu::SurfaceError> {
        // Get the surface texture we will draw on
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Encoder will send commands to the queue
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        {
            let pipeline = renderer::get_pipeline(self);
            let mut pass =
                renderer::get_render_pass(&mut encoder, &state, &view);
            pass.set_pipeline(&pipeline);
            renderer::draw(&mut pass, &state);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
