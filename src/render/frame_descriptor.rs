use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Color, Device,
};

use crate::{
    gpu_primitives::{GpuPrimitive, GpuQuad, GpuTransform},
    render::camera::Camera,
    sim::{world, State},
};

pub struct FrameDescriptor {
    wireframe: bool,
    transforms: Vec<GpuTransform>,
    camera: Camera,
    pub clear_color: Color,
}

impl FrameDescriptor {
    pub fn from(state: &State) -> FrameDescriptor {
        let mut transforms = Vec::new();

        for body in &state.bodies {
            transforms.push(body.into())
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
            transforms,
            camera,
            clear_color,
        }
    }

    pub fn indicies(&self) -> u32 {
        match self.wireframe {
            true => 1 as u32 * 5,
            false => 1 as u32 * 6,
        }
    }

    pub fn instances(&self) -> u32 {
        self.transforms.len() as u32
    }

    pub fn create_vertex_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &self.get_vertex_buffer_contents(),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn get_vertex_buffer_contents(&self) -> Vec<u8> {
        GpuQuad.data()
    }

    pub fn create_index_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.get_index_buffer_contents()),
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    pub fn create_instance_buffer(&self, device: &Device) -> Buffer {
        let instance_data = self
            .transforms
            .iter()
            .map(|gpu_transform| gpu_transform.model)
            .collect::<Vec<_>>();

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn get_index_buffer_contents(&self) -> Vec<u16> {
        match self.wireframe {
            true => vec![0, 1, 2, 3, 0],
            false => vec![0, 1, 2, 0, 2, 3],
        }
    }

    pub fn create_camera_binding(
        &self,
        device: &Device,
    ) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout) {
        self.camera.bind(device)
    }

    pub fn create_world_radius_binding(
        &self,
        device: &Device,
    ) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout) {
        world::bind_world_uniform(device)
    }
}
