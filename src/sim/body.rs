use glam::Vec2;
use rapier2d::prelude::*;

use crate::sim::{GRAVITY_AMPLIFIER, UNIVERSAL_GRAVITY};

use super::particle::{AsParticle, Particle, ParticleSet};

#[derive(Default)]
pub struct Body {
    position: Vec2,
    rotation: f32,
    radius: f32,
    mass: f32,
    pub rigidbody_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
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

impl AsParticle for Body {
    fn as_particle(&self) -> Particle {
        Particle::new(
            self.position(),
            self.mass() * UNIVERSAL_GRAVITY * GRAVITY_AMPLIFIER,
        )
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

    fn sync_to_rigidbody(
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

    fn apply_force_to_rigidbodies(
        &self,
        bodies: &mut RigidBodySet,
        force: Vec2,
    ) {
        let rb = bodies.get_mut(self.rigidbody_handle).unwrap();

        rb.reset_forces(true);
        rb.add_force(force.into(), true);
    }
}

pub trait RigidBodies {
    fn update(&mut self, bodies: &mut RigidBodySet, colliders: &ColliderSet);
}

impl RigidBodies for ParticleSet<Body> {
    fn update(&mut self, bodies: &mut RigidBodySet, colliders: &ColliderSet) {
        // First we sync each gravitational body to its corresponding rigidbody and collider
        for particle in &mut self.particles {
            particle.sync_to_rigidbody(bodies, colliders);
        }

        // Then we can calculate and apply the force
        for (particle, acceleration) in
            self.particles.iter().zip(self.get_accelerations())
        {
            particle.apply_force_to_rigidbodies(
                bodies,
                acceleration * particle.mass(),
            )
        }
    }
}
