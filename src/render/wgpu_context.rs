use std::{collections::HashMap, io::BufReader, path::Path};

use gloo_console::log;
use std::fs::File;
use std::io::Read;
use wgpu::{Sampler, ShaderModule, Texture, TextureView};
use winit::window::Window;

use crate::sim::State;

use super::{frame_descriptor::FrameDescriptor, pipelines};

pub struct WgpuContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    shaders: HashMap<&'static str, ShaderModule>,
    textures: HashMap<&'static str, (Texture, TextureView, Sampler)>,
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

        Self {
            surface,
            device,
            queue,
            config,
            size,
            shaders: HashMap::new(),
            textures: HashMap::new(),
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

        let frame_desc = FrameDescriptor::from(&state);
        let vertex_buffer = frame_desc.get_vertex_buffer(&self.device);
        let index_buffer = frame_desc.get_index_buffer(&self.device);
        {
            let pipeline = match &state.wireframe {
                true => pipelines::Pipeline::Wireframe,
                false => pipelines::Pipeline::Solid,
            }
            .get(self);

            let color = wgpu::Color {
                r: state.bg_color.x,
                g: state.bg_color.y,
                b: state.bg_color.z,
                a: 1.0,
            };

            let mut pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(color),
                                store: true,
                            },
                        },
                    )],
                    depth_stencil_attachment: None,
                });
            pass.set_pipeline(&pipeline);
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.set_index_buffer(
                index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            pass.draw_indexed(
                0..frame_desc.indicies(),
                0,
                0..frame_desc.instances(),
            );
        }

        // Submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn add_shader<P: AsRef<Path>>(&mut self, name: &'static str, path: P) {
        let path = path.as_ref().to_str().expect("Path not found");

        let shader_module =
            self.device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(name),
                    source: wgpu::ShaderSource::Wgsl(
                        std::fs::read_to_string(path).unwrap().into(),
                    ),
                });

        self.shaders.insert(name, shader_module);
    }

    pub fn add_texture<P: AsRef<Path>>(&mut self, name: &'static str, path: P) {
        let f = File::open(path).expect("Can't open texture file");
        let mut reader = BufReader::new(f);
        let mut bytes = Vec::new();

        // Read file into vector.
        reader
            .read_to_end(&mut bytes)
            .expect("Can't read texture data from file");

        let image = image::load_from_memory(&bytes).unwrap();
        let rgba = image.to_rgba8();

        use image::GenericImageView;
        let dimensions = image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            label: Some(name),
        });

        self.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &rgba,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            texture_size,
        );

        let texture_view =
            texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        self.textures.insert(name, (texture, texture_view, sampler));
    }
}
