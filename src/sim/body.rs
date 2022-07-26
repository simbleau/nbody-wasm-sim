use glam::{Quat, Vec2};

use crate::render::RenderInstance;

#[derive(Clone, Debug)]
pub struct Body {
    pub origin: Vec2,
    pub rotation: f32,
    pub elapsed: f32,
}

impl Default for Body {
    fn default() -> Self {
        Body {
            origin: Vec2::new(0.0, 0.0),
            elapsed: 0.0,
            rotation: 0.0,
        }
    }
}

impl Body {
    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
    }
}

impl From<&Body> for RenderInstance {
    fn from(body: &Body) -> Self {
        RenderInstance {
            position: body.origin,
            rotation: Quat::from_rotation_z(body.rotation),
        }
    }
}
