use std::collections::HashMap;

use gloo_console::log;
use wgpu::{BindGroup, BindGroupLayout, ShaderModule, Texture};
use winit::dpi::PhysicalSize;

use crate::render::{frame_descriptor::FrameDescriptor, pipelines::Pipeline};
use crate::sim::{State, WORLD_EDGE_SEGMENTS};

pub struct WgpuContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    shaders: HashMap<&'static str, ShaderModule>,
    textures: HashMap<&'static str, (Texture, BindGroup, BindGroupLayout)>,
}

impl WgpuContext {
    // Creating some of the wgpu types requires async code
    pub async fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
        let (width, height) = (canvas.width(), canvas.height());
        log!("Surface size:", width, height);

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = instance.create_surface_from_canvas(canvas);

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
            width,
            height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size: PhysicalSize::new(width, height),
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

        // Create all data from the state we need for a frame
        let frame_desc = FrameDescriptor::build(&state);
        let vertex_buffer = frame_desc.create_vertex_buffer(&self.device);
        let index_buffer = frame_desc.create_index_buffer(&self.device);
        let (
            camera_buffer,
            camera_buffer_contents,
            camera_bind_group,
            camera_bind_group_layout,
        ) = frame_desc.create_camera_binding(&self.device);
        let (_, tex_bind_group, tex_bind_group_layout) =
            self.get_texture(state.texture_key);
        let instance_buffer = frame_desc.create_instance_buffer(&self.device);
        // Get rendering pipeline
        let pipeline = match &state.wireframe {
            true => {
                let pipeline_layout = self.device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: Some("Solid Pipeline Layout"),
                        bind_group_layouts: &[&camera_bind_group_layout],
                        push_constant_ranges: &[],
                    },
                );
                Pipeline::Wireframe.get(self, pipeline_layout)
            }
            false => {
                let pipeline_layout = self.device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: Some("Wireframe Pipeline Layout"),
                        bind_group_layouts: &[
                            &camera_bind_group_layout,
                            tex_bind_group_layout,
                        ],
                        push_constant_ranges: &[],
                    },
                );
                Pipeline::Solid.get(self, pipeline_layout)
            }
        };

        // Data for world boundaries
        let (
            wradius_buffer,
            world_buffer_contents,
            wradius_bind_group,
            wradius_bind_group_layout,
        ) = frame_desc.create_world_data_binding(&self.device);
        let world_pipeline = {
            let pipeline_layout = self.device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: Some("World Pipeline Layout"),
                    bind_group_layouts: &[
                        &camera_bind_group_layout,
                        &wradius_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                },
            );
            Pipeline::World.get(self, pipeline_layout)
        };

        // Execute render pass
        {
            // Make pass
            let mut pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(
                                    frame_desc.clear_color,
                                ),
                                store: true,
                            },
                        },
                    )],
                    depth_stencil_attachment: None,
                });

            // Draw world data
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &camera_bind_group, &[]);
            if !state.wireframe {
                pass.set_bind_group(1, tex_bind_group, &[]);
            }
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.set_vertex_buffer(1, instance_buffer.slice(..));
            pass.set_index_buffer(
                index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            pass.draw_indexed(
                0..frame_desc.indicies().len() as u32,
                0,
                0..frame_desc.instances().len() as u32,
            );

            // Draw world boundaries
            pass.set_pipeline(&world_pipeline);
            pass.set_bind_group(0, &camera_bind_group, &[]);
            pass.set_bind_group(1, &wradius_bind_group, &[]);
            pass.draw(0..(WORLD_EDGE_SEGMENTS + 1), 0..1);
        }

        // Write buffers
        self.queue
            .write_buffer(&camera_buffer, 0, &camera_buffer_contents);
        self.queue
            .write_buffer(&wradius_buffer, 0, &world_buffer_contents);

        // Submit queue
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(output.present())
    }

    pub fn add_shader(&mut self, name: &'static str, source: &'static str) {
        if self.shaders.contains_key(name) {
            panic!("Shader with name '{}' already exists", name);
        }
        let shader = crate::render::Shader::new(name, source);
        self.shaders.insert(name, shader.bind(&self.device));
    }

    pub fn add_texture(&mut self, name: &'static str, bytes: &'static [u8]) {
        if self.textures.contains_key(name) {
            panic!("Texture with name '{}' already exists", name);
        }
        let sw_texture = crate::render::Texture::new(name, bytes);
        let (hw_texture, bind_group, layout) = sw_texture.bind(&self.device);
        self.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &hw_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &sw_texture.rgba,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(
                    4 * sw_texture.dimensions.0,
                ),
                rows_per_image: std::num::NonZeroU32::new(
                    sw_texture.dimensions.1,
                ),
            },
            sw_texture.size,
        );
        self.textures.insert(name, (hw_texture, bind_group, layout));
    }

    pub fn get_shader(&self, name: &'static str) -> &ShaderModule {
        self.shaders
            .get(name)
            .expect(&format!("No shader with name '{}'", name))
    }

    pub fn get_texture<'a>(
        &self,
        name: &'a str,
    ) -> &(Texture, BindGroup, BindGroupLayout) {
        self.textures
            .get(name)
            .expect(&format!("No texture with name '{}'", name))
    }
}
