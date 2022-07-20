use wgpu::{util::DeviceExt, Buffer, Device};

use crate::{gpu_primitives::GpuTriangle, sim::{State, WORLD_SIZE}};

use super::camera::{Camera};

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

        let camera = Camera::new(state.window_size.as_vec2(), WORLD_SIZE, state.pan, state.zoom);

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
}
