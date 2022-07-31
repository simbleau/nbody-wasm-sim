use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

use crate::gpu_primitives::{
    GpuPrimitive, WorldBoundaryUniform, WorldRadiusUniform,
};

pub const WORLD_RADIUS: f32 = 50.0;
pub const WORLD_EDGE_SEGMENTS: u32 = 4;

pub fn bind_world_radius(
    device: &Device,
) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout) {
    let layout = create_wradius_bind_group_layout(device);
    let buffer_contents = get_wradius_buffer_contents();
    let buffer = create_wradius_buffer(device, &buffer_contents);
    let bind_group = create_wradius_bind_group(&buffer, &layout, device);
    (buffer, buffer_contents, bind_group, layout)
}

fn get_wradius_buffer_contents() -> Vec<u8> {
    let uniform = WorldRadiusUniform {
        radius: WORLD_RADIUS,
    };
    uniform.data()
}

fn create_wradius_bind_group(
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
        label: Some("World Radius Bind Group"),
    })
}

fn create_wradius_buffer(device: &Device, buffer_contents: &[u8]) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("World Radius Buffer"),
        contents: buffer_contents,
        usage: wgpu::BufferUsages::UNIFORM
            | wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST,
    })
}

fn create_wradius_bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("World Radius Bind Group Layout"),
    })
}

pub fn bind_world_boundaries(
    device: &Device,
) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout) {
    let layout = create_wbounds_bind_group_layout(device);
    let buffer_contents = get_wbounds_buffer_contents();
    let buffer = create_wbounds_buffer(device, &buffer_contents);
    let bind_group = create_wbounds_bind_group(&buffer, &layout, device);
    (buffer, buffer_contents, bind_group, layout)
}

fn get_wbounds_buffer_contents() -> Vec<u8> {
    WorldBoundaryUniform.data()
}

fn create_wbounds_bind_group(
    buffer: &Buffer,
    layout: &BindGroupLayout,
    device: &Device,
) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 1,
            resource: buffer.as_entire_binding(),
        }],
        label: Some("World Boundaries Bind Group"),
    })
}

fn create_wbounds_buffer(device: &Device, buffer_contents: &[u8]) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("World Boundaries Buffer"),
        contents: buffer_contents,
        usage: wgpu::BufferUsages::UNIFORM
            | wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST,
    })
}

fn create_wbounds_bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("World Boundaries Bind Group Layout"),
    })
}
