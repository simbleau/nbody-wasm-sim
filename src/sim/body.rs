use glam::{Mat4, Quat, Vec2};

use crate::gpu_primitives::GpuTransform;
#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    pub origin: Vec2,
    pub radius: f32,
    pub rotation: f32,
    pub elapsed: f32,
    pub density: f32,
    pub init_velocity: Vec2,
    pub velocity: Vec2,
}

impl Default for Body {
    fn default() -> Self {
        Body {
            origin: Vec2::new(0.0, 0.0),
            radius: 0.5,
            elapsed: 0.0,
            rotation: 0.0,
            init_velocity: Vec2::ZERO,
            velocity: Vec2::ZERO,
            density: 1.0,
        }
    }
}

impl Body {
    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    pub fn mass(&self) -> f32 {
        self.area() * self.density
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
    }

    pub fn new(origin: Vec2, rotation: f32, radius: f32) -> Self {
        Self {
            origin,
            radius,
            rotation,
            ..Default::default()
        }
    }
}

impl From<&Body> for GpuTransform {
    fn from(body: &Body) -> Self {
        GpuTransform {
            model: Mat4::from_scale_rotation_translation(
                Vec2::splat(2.0 * body.radius).extend(1.0),
                Quat::from_rotation_z(body.rotation),
                body.origin.extend(1.0),
            )
            .to_cols_array_2d(),
        }
    }
}
