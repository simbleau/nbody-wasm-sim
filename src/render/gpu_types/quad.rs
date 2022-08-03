use crate::render::gpu_types::{GpuPrimitive, GpuVertex};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GpuQuad;

impl GpuPrimitive for GpuQuad {
    fn data(&self) -> Vec<u8> {
        const VERTICES: &[GpuVertex] = &[
            GpuVertex {
                position: [0.5, 0.5, 0.0],
                uv: [1.0, 0.0],
            },
            GpuVertex {
                position: [-0.5, 0.5, 0.0],
                uv: [0.0, 0.0],
            },
            GpuVertex {
                position: [-0.5, -0.5, 0.0],
                uv: [0.0, 1.0],
            },
            GpuVertex {
                position: [0.5, -0.5, 0.0],
                uv: [1.0, 1.0],
            },
        ];
        bytemuck::cast_slice(VERTICES).to_vec()
    }

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        GpuVertex::desc()
    }
}

unsafe impl bytemuck::Pod for GpuQuad {}
unsafe impl bytemuck::Zeroable for GpuQuad {}
