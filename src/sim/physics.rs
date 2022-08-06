use glam::Vec2;
use rapier2d::prelude::*;

use super::{Body, UNIVERSAL_GRAVITY};

pub struct PhysicsContext {
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub ccd_solver: CCDSolver,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
}

impl PhysicsContext {
    pub fn update(&mut self, bodies: &Vec<Body>, dt: f32) {
        self.integration_parameters.dt = dt;

        // Calculate velocity vectors
        let num_bodies = bodies.len();
        for i in 0..num_bodies {
            // Get displacement
            let body = &bodies[i];
            let mut force = Vec2::ZERO;
            for other in bodies {
                if body != other {
                    let sqr_dist = (other.position(&self)
                        - body.position(&self))
                    .length_squared();
                    let force_dir = (other.position(&self)
                        - body.position(&self))
                    .normalize();
                    force += force_dir
                        * UNIVERSAL_GRAVITY
                        * body.mass(&self)
                        * other.mass(&self)
                        / sqr_dist;
                }
            }

            // Apply gravity
            let rigid_body =
                self.rigid_body_set.get_mut(body.rigid_body_handle).unwrap();
            rigid_body.reset_forces(true);
            rigid_body.add_force(vector![force.x, force.y], true);
        }

        self.physics_pipeline.step(
            &vector![0.0, 0.0],
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut ImpulseJointSet::new(),
            &mut MultibodyJointSet::new(),
            &mut self.ccd_solver,
            &(),
            &(),
        );
    }
}
