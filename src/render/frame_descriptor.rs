use glam::{Mat4, Quat, Vec2};
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Color, Device,
};

use crate::{
    render::camera::Camera,
    render::gpu_types::{
        CameraUniform, GpuPrimitive, GpuQuad, GpuTransform, GpuUniform,
        WorldUniform,
    },
    sim::Simulation,
};

pub struct FrameDescriptor {
    wireframe: bool,
    transforms: Vec<GpuTransform>,
    camera: Camera,
    pub clear_color: Color,
}

impl FrameDescriptor {
    pub fn build(sim: &Simulation) -> FrameDescriptor {
        let mut transforms = Vec::new();
        for body in &sim.bodies {
            transforms.push(GpuTransform {
                model: Mat4::from_scale_rotation_translation(
                    Vec2::splat(2.0 * body.radius(&sim.physics_context))
                        .extend(1.0),
                    Quat::from_rotation_z(body.rotation(&sim.physics_context)),
                    body.position(&sim.physics_context).extend(1.0),
                )
                .to_cols_array_2d(),
            })
        }

        let camera = Camera::new(
            sim.state.view_size.as_vec2(),
            sim.state.rotation,
            sim.state.pan,
            sim.state.zoom,
        );

        let clear_color = Color {
            r: sim.state.bg_color.x,
            g: sim.state.bg_color.y,
            b: sim.state.bg_color.z,
            a: 1.0,
        };

        FrameDescriptor {
            wireframe: sim.state.wireframe,
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
