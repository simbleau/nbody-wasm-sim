use crate::sim::world::WORLD_EDGE_SEGMENTS;

use super::{GpuPrimitive, GpuVertex};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WorldBoundaryUniform;

impl GpuPrimitive for WorldBoundaryUniform {
    fn data(&self) -> Vec<u8> {
        let delta_radians = std::f64::consts::PI / WORLD_EDGE_SEGMENTS as f64;
        let mut verts = vec![];
        for i in 0..WORLD_EDGE_SEGMENTS {
            let x = (delta_radians * i as f64).cos() as f32;
            let y = (delta_radians * i as f64).sin() as f32;
            let vert = GpuVertex {
                position: [x, y, 0.0],
                uv: [0.0, 0.0], // TODO: Decouple UV from GpuVertex
            };
            verts.push(vert);
        }
        verts.push(GpuVertex {
            position: [1.0, 0.0, 0.0],
            uv: [0.0, 0.0],
        });

        bytemuck::cast_slice(&verts).to_vec()
    }

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        GpuVertex::desc()
    }
}

unsafe impl bytemuck::Pod for WorldBoundaryUniform {}
unsafe impl bytemuck::Zeroable for WorldBoundaryUniform {}
