use glam::Vec2;
use nalgebra::{Complex, Unit};
use rapier2d::prelude::*;

use super::physics::PhysicsContext;

#[derive(PartialEq)]
pub struct Body {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl Body {
    pub fn mass(&self, ctx: &PhysicsContext) -> f32 {
        ctx.rigid_body_set
            .get(self.rigid_body_handle)
            .unwrap()
            .mass()
    }

    pub fn radius(&self, ctx: &PhysicsContext) -> f32 {
        ctx.collider_set
            .get(self.collider_handle)
            .map(Collider::shape)
            .map(<dyn shape::Shape>::as_ball)
            .flatten()
            .map(|ball| ball.radius)
            .unwrap()
    }

    pub fn rotation(&self, ctx: &PhysicsContext) -> f32 {
        ctx.rigid_body_set
            .get(self.rigid_body_handle)
            .map(RigidBody::rotation)
            .map(Unit::<Complex<f32>>::angle)
            .unwrap()
    }

    pub fn position(&self, ctx: &PhysicsContext) -> Vec2 {
        ctx.rigid_body_set
            .get(self.rigid_body_handle)
            .map(RigidBody::position)
            .map(|p| Vec2::new(p.translation.x, p.translation.y))
            .unwrap()
    }
}
