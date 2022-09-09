use glam::{Vec2, Vec3};
use particular::prelude::Particle;
use rapier2d::prelude::*;

use crate::sim::{GRAVITY_AMPLIFIER, UNIVERSAL_GRAVITY};

#[derive(Default)]
pub struct Body {
    position: Vec2,
    rotation: f32,
    radius: f32,
    mass: f32,
    pub rigidbody_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl Particle for Body {
    fn position(&self) -> Vec3 {
        self.position.extend(0.0)
    }

    fn mu(&self) -> f32 {
        self.mass * GRAVITY_AMPLIFIER * UNIVERSAL_GRAVITY
    }
}

impl Body {
    pub fn new(
        rigidbody_handle: RigidBodyHandle,
        collider_handle: ColliderHandle,
    ) -> Self {
        Self {
            rigidbody_handle,
            collider_handle,
            ..Default::default()
        }
    }
}

impl Body {
    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn mass(&self) -> f32 {
        self.mass
    }

    pub fn sync_to_rigidbody(
        &mut self,
        bodies: &RigidBodySet,
        colliders: &ColliderSet,
    ) {
        let rb = bodies.get(self.rigidbody_handle).unwrap();
        let coll = colliders.get(self.collider_handle).unwrap();

        self.position = (*rb.translation()).into();
        self.rotation = rb.rotation().angle();
        self.radius = coll.shape().as_ball().unwrap().radius;
        self.mass = rb.mass();
    }

    pub fn apply_acceleration_to_rigidbody(
        &self,
        bodies: &mut RigidBodySet,
        acceleration: Vec2,
    ) {
        let rb = bodies.get_mut(self.rigidbody_handle).unwrap();
        let force = acceleration * self.mass();

        rb.reset_forces(true);
        rb.add_force(force.into(), true);
    }
}
