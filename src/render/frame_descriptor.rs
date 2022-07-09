use wgpu::{util::DeviceExt, Buffer, Device};

use crate::{gpu_primitives::GpuTriangle, sim::State};

pub struct FrameDescriptor {
    wireframe: bool,
    gpu_triangles: Vec<GpuTriangle>,
}

impl FrameDescriptor {
    pub fn from(state: &State) -> FrameDescriptor {
        let mut gpu_triangles = Vec::new();

        for body in &state.bodies {
            gpu_triangles.push(body.into())
        }

        FrameDescriptor {
            wireframe: state.wireframe,
            gpu_triangles,
        }
    }

    pub fn verticies(&self) -> u32 {
        match self.wireframe {
            true => self.gpu_triangles.len() as u32 * 3 + 1,
            false => self.gpu_triangles.len() as u32 * 3,
        }
    }

    pub fn instances(&self) -> u32 {
        1
    }

    pub fn as_vertex_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &self.as_vertex_buffer_contents(),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn as_vertex_buffer_contents(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        for instance in self.gpu_triangles.iter() {
            let bytes = bytemuck::cast_slice(&instance.verts);
            buf.extend(bytes);
        }

        if let true = self.wireframe {
            let first_vert = &[self.gpu_triangles[0].verts[0]];
            buf.extend(bytemuck::cast_slice(first_vert));
        }

        buf
    }
}
