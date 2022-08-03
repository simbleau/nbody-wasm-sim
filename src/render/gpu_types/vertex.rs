use wgpu::VertexBufferLayout;

use crate::render::gpu_types::GpuPrimitive;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GpuVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

unsafe impl bytemuck::Pod for GpuVertex {}
unsafe impl bytemuck::Zeroable for GpuVertex {}

impl GpuVertex {
    pub const BUFFER_LAYOUT: VertexBufferLayout<'static> =
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GpuVertex>()
                as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2],
        };
}

impl GpuPrimitive for GpuVertex {
    fn data(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[*self]).to_vec()
    }

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        Self::BUFFER_LAYOUT
    }
}
