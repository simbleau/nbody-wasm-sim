use super::{GpuPrimitive, GpuVertex};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GpuTriangle {
    pub verts: [GpuVertex; 3],
}

impl GpuPrimitive for GpuTriangle {
    fn data(&self) -> Vec<u8> {
        bytemuck::cast_slice(&self.verts).to_vec()
    }

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        GpuVertex::BUFFER_LAYOUT
    }
}

unsafe impl bytemuck::Pod for GpuTriangle {}
unsafe impl bytemuck::Zeroable for GpuTriangle {}
