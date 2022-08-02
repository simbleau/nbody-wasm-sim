use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

use crate::{gpu_types::GpuUniform, render::Camera};

pub struct CameraUniform<'a> {
    pub camera: &'a Camera,
}

impl<'a> From<&'a Camera> for CameraUniform<'a> {
    fn from(c: &'a Camera) -> Self {
        CameraUniform { camera: c }
    }
}

impl<'a> GpuUniform for CameraUniform<'a> {
    fn bind(
        &self,
        device: &Device,
    ) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout) {
        let layout = self.create_bind_group_layout(device);
        let buffer_contents = self.get_buffer_contents();
        let buffer = self.create_buffer(device, &buffer_contents);
        let bind_group = self.create_bind_group(&buffer, &layout, device);
        (buffer, buffer_contents, bind_group, layout)
    }
}

impl<'a> CameraUniform<'a> {
    fn get_buffer_contents(&self) -> Vec<u8> {
        let matrix = self
            .camera
            .build_view_projection_matrix()
            .to_cols_array_2d();
        bytemuck::cast_slice(&[matrix]).to_vec()
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
