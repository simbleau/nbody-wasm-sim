use wgpu::VertexBufferLayout;

use super::GpuPrimitive;

#[derive(Copy, Clone, Debug)]
pub struct GpuCircle {
    pub position: [f32; 2],
    pub radius: f32,
}

impl GpuCircle {
    pub const BUFFER_LAYOUT: VertexBufferLayout<'static> =
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GpuCircle>()
                as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32],
        };
}

impl GpuPrimitive for GpuCircle {
    fn data(&self) -> Vec<u8> {
        let buf = vec![self.position[0], self.position[1], self.radius];
        bytemuck::cast_slice(&buf).to_vec()
    }

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        Self::BUFFER_LAYOUT
    }
}
