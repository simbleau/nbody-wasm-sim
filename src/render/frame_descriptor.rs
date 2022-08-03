use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Color, Device,
};

use crate::{
    gpu_types::{
        CameraUniform, GpuPrimitive, GpuQuad, GpuTransform, GpuUniform,
        WorldUniform,
    },
    render::camera::Camera,
    sim::State,
};

pub struct FrameDescriptor {
    wireframe: bool,
    transforms: Vec<GpuTransform>,
    camera: Camera,
    pub clear_color: Color,
}

impl FrameDescriptor {
    pub fn build(state: &State) -> FrameDescriptor {
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

    pub fn indicies(&self) -> Vec<u16> {
        match self.wireframe {
            true => vec![0, 1, 2, 3, 0],
            false => vec![0, 1, 2, 0, 2, 3],
        }
    }

    pub fn instances(&self) -> &Vec<GpuTransform> {
        &self.transforms
    }

    pub fn create_vertex_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &GpuQuad.data(),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn create_index_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indicies()),
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

    pub fn create_camera_binding(
        &self,
        device: &Device,
    ) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout) {
        CameraUniform::from(&self.camera).bind(device)
    }

    pub fn create_world_data_binding(
        &self,
        device: &Device,
    ) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout) {
        WorldUniform::default().bind(device)
    }
}
