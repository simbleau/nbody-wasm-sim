use glam::{Mat4, Quat, Vec2};
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

use crate::gpu_primitives::CameraUniform;

pub struct Camera {
    scale: f32,
    rotation: f32,
    translation: Vec2,
    view_size: Vec2,
}

impl Camera {
    pub fn new(
        view_size: Vec2,
        rotation: f32,
        translation: Vec2,
        scale: f32,
    ) -> Self {
        Camera {
            scale,
            rotation,
            translation,
            view_size,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::from_rotation_translation(
            Quat::from_rotation_z(self.rotation),
            self.translation.extend(1.0),
        );

        let (width, height) = self.view_size.into();
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        let left = -half_width;
        let right = half_width;
        let top = half_height;
        let bottom = -half_height;

        let proj = Mat4::orthographic_rh(
            left / self.scale,
            right / self.scale,
            bottom / self.scale,
            top / self.scale,
            0.0,
            1.0,
        );

        return proj * view.inverse();
    }

    pub fn bind(
        &self,
        device: &Device,
    ) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout) {
        let layout = self.create_bind_group_layout(device);
        let buffer_contents = self.get_buffer_contents();
        let buffer = self.create_buffer(device, &buffer_contents);
        let bind_group = self.create_bind_group(&buffer, &layout, device);
        (buffer, buffer_contents, bind_group, layout)
    }

    fn get_buffer_contents(&self) -> Vec<u8> {
        let matrix = self.build_view_projection_matrix().to_cols_array_2d();
        let camera_uniform = CameraUniform { view_proj: matrix };
        bytemuck::cast_slice(&[camera_uniform]).to_vec()
    }

    fn create_bind_group(
        &self,
        buffer: &Buffer,
        layout: &BindGroupLayout,
        device: &Device,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        })
    }

    fn create_buffer(&self, device: &Device, buffer_contents: &[u8]) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: buffer_contents,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    fn create_bind_group_layout(&self, device: &Device) -> BindGroupLayout {
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
}
