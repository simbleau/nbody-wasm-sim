use glam::Vec2;
use rapier2d::prelude::*;

use crate::sim::{GRAVITY_AMPLIFIER, UNIVERSAL_GRAVITY};

pub struct BodySet {
    pub particles: Vec<Body>,
}

impl BodySet {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }
}

impl BodySet {
    pub fn get_accelerations(&self) -> Vec<Vec2> {
        let accelerations = self.particles.iter().map(|particle1| {
            self.particles
                .iter()
                .fold(Vec2::ZERO, |acceleration, particle2| {
                    let dir = particle2.position() - particle1.position();
                    let mag_2 = dir.length_squared();

                    let grav_acc = if mag_2 != 0.0 {
                        UNIVERSAL_GRAVITY
                            * GRAVITY_AMPLIFIER
                            * particle2.mass()
                            * dir
                            / (mag_2 * mag_2.sqrt())
                    } else {
                        dir
                    };

                    acceleration + grav_acc
                })
        });

        accelerations.collect()
    }

    /// Used to define the behaviour of the particles using their computed gravitational acceleration.
    pub fn response<F>(&mut self, response: F)
    where
        F: FnOnce(Vec<(&mut Body, Vec2)>),
    {
        let accelerations = self.get_accelerations();
        response(self.particles.iter_mut().zip(accelerations).collect());
    }
}

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

    pub fn apply_force_to_rigidbody(
        &self,
        bodies: &mut RigidBodySet,
        force: Vec2,
    ) {
        let rb = bodies.get_mut(self.rigidbody_handle).unwrap();

        rb.reset_forces(true);
        rb.add_force(force.into(), true);
    }
}
