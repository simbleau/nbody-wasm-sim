use glam::{Mat4, Quat, Vec2};

use crate::gpu_primitives::InstanceRaw;

pub struct RenderInstance {
    position: Vec2,
    rotation: Quat,
}

// NEW!
impl RenderInstance {
    fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: Mat4::from_rotation_translation(
                self.rotation,
                self.position.extend(0.0),
            )
            .to_cols_array_2d(),
        }
    }
}
