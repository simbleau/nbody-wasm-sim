use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

use crate::{
    gpu_primitives::{CameraUniform, GpuTriangle},
    sim::State,
};

use super::camera::Camera;

pub struct FrameDescriptor {
    wireframe: bool,
    gpu_triangles: Vec<GpuTriangle>,
    camera: Camera,
}

impl FrameDescriptor {
    pub fn from(state: &State) -> FrameDescriptor {
        let mut gpu_triangles = Vec::new();

        for body in &state.bodies {
            gpu_triangles.push(body.into())
        }

        let camera =
            Camera::new(state.view_size.as_vec2(), state.pan, state.zoom);
        gloo_console::log!(
            "view",
            state.view_size.as_vec2().x,
            state.view_size.as_vec2().y
        );
        gloo_console::log!("translation", state.pan.x, state.pan.y);
        gloo_console::log!("scale", state.zoom);

        FrameDescriptor {
            wireframe: state.wireframe,
            gpu_triangles,
            camera,
        }
    }

    pub fn indicies(&self) -> u32 {
        match self.wireframe {
            true => self.gpu_triangles.len() as u32 * 3 + 1,
            false => self.gpu_triangles.len() as u32 * 3,
        }
    }

    pub fn instances(&self) -> u32 {
        1
    }

    pub fn get_vertex_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &self.get_vertex_buffer_contents(),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn get_vertex_buffer_contents(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        for instance in self.gpu_triangles.iter() {
            let bytes = bytemuck::cast_slice(&instance.verts);
            buf.extend(bytes);
        }

        buf
    }

    pub fn get_index_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.get_index_buffer_contents()),
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    pub fn get_index_buffer_contents(&self) -> Vec<u16> {
        let mut buf: Vec<u16> = Vec::new();

        let stride = match self.wireframe {
            true => 4,
            false => 3,
        };

        for (i, _) in self.gpu_triangles.iter().enumerate() {
            let indx = i as u16 * stride;

            buf.push(indx);
            buf.push(indx + 1);
            buf.push(indx + 2);

            if let true = self.wireframe {
                buf.push(indx);
            }
        }

        buf
    }

    pub fn get_camera_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: &self.get_camera_buffer_contents(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    pub fn get_camera_buffer_contents(&self) -> Vec<u8> {
        let matrix = self
            .camera
            .build_view_projection_matrix()
            .to_cols_array_2d();
        gloo_console::log!("matrix", format!("{:#?}", matrix));
        let camera_uniform = CameraUniform { view_proj: matrix };
        bytemuck::cast_slice(&[camera_uniform]).to_vec()
    }

    pub fn get_camera_bind_group_layout(
        &self,
        device: &Device,
    ) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Camera Bind Group Layout"),
        })
    }

    pub fn get_camera_bind_group(
        &self,
        camera_buffer: &Buffer,
        layout: &BindGroupLayout,
        device: &Device,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        })
    }
}
