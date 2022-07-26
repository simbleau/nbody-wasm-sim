use glam::{Mat4, Quat, Vec2};

use crate::gpu_primitives::GpuTransform;

pub struct RenderInstance {
    pub position: Vec2,
    pub rotation: Quat,
}

impl RenderInstance {
    pub fn to_raw(&self) -> GpuTransform {
        GpuTransform {
            model: Mat4::from_rotation_translation(
                self.rotation,
                self.position.extend(0.0),
            )
            .to_cols_array_2d(),
        }
    }
}
