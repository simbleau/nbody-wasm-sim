use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Color, Device,
};

use crate::{gpu_primitives::GpuTriangle, sim::State};

use super::camera::Camera;

pub struct FrameDescriptor {
    wireframe: bool,
    gpu_triangles: Vec<GpuTriangle>,
    camera: Camera,
    pub clear_color: Color,
}

impl FrameDescriptor {
    pub fn from(state: &State) -> FrameDescriptor {
        let mut gpu_triangles = Vec::new();

        for body in &state.bodies {
            gpu_triangles.push(body.into())
        }

        let camera = Camera::new(
            state.view_size.as_vec2(),
            state.rotation,
            state.pan,
            state.zoom,
        );

        let clear_color = Color {
            r: state.bg_color.x,
            g: state.bg_color.y,
            b: state.bg_color.z,
            a: 1.0,
        };

        FrameDescriptor {
            wireframe: state.wireframe,
            gpu_triangles,
            camera,
            clear_color,
        }
    }

    pub fn indicies(&self) -> u32 {
        match self.wireframe {
            true => self.gpu_triangles.len() as u32 * 3 + 1,
            false => self.gpu_triangles.len() as u32 * 3,
        }
    }

    pub fn instances(&self) -> u32 {
        // TODO
        1
    }

    pub fn create_vertex_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &self.get_vertex_buffer_contents(),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn get_vertex_buffer_contents(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        for instance in self.gpu_triangles.iter() {
            let bytes = bytemuck::cast_slice(&instance.verts);
            buf.extend(bytes);
        }

        buf
    }

    pub fn create_index_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.get_index_buffer_contents()),
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    fn get_index_buffer_contents(&self) -> Vec<u16> {
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

    pub fn create_camera_binding(
        &self,
        device: &Device,
    ) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout) {
        self.camera.bind(device)
    }
}
