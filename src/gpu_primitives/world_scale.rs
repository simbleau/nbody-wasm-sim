use wgpu::VertexBufferLayout;

use super::GpuPrimitive;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WorldRadiusUniform {
    pub radius: f32,
}

unsafe impl bytemuck::Pod for WorldRadiusUniform {}
unsafe impl bytemuck::Zeroable for WorldRadiusUniform {}

impl WorldRadiusUniform {
    pub const BUFFER_LAYOUT: VertexBufferLayout<'static> =
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<WorldRadiusUniform>()
                as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32],
        };
}

impl GpuPrimitive for WorldRadiusUniform {
    fn data(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[*self]).to_vec()
    }

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        Self::BUFFER_LAYOUT
    }
}
