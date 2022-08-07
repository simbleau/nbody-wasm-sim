use glam::Vec2;
use rapier2d::prelude::*;

use super::{Body, GRAVITY_AMPLIFIER, UNIVERSAL_GRAVITY};

pub struct PhysicsContext {
    pub bodies: Vec<Body>,
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
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            ccd_solver: CCDSolver::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
        }
    }

    pub fn create_body(
        &mut self,
        rb: impl Into<RigidBody>,
        coll: impl Into<Collider>,
    ) {
        let rigid_body_handle = self.rigid_body_set.insert(rb);
        let collider_handle = self.collider_set.insert_with_parent(
            coll,
            rigid_body_handle,
            &mut self.rigid_body_set,
        );
        let body = Body {
            rigid_body_handle,
            collider_handle,
        };
        self.bodies.push(body);
    }

    pub fn step(&mut self) {
        // Calculate velocity vectors
        let num_bodies = self.bodies.len();
        for i in 0..num_bodies {
            // Get displacement
            let body = &self.bodies[i];
            let mut force = Vec2::ZERO;
            for other in &self.bodies {
                if body != other {
                    let sqr_dist = (other.position(self) - body.position(self))
                        .length_squared();
                    let force_dir = (other.position(self)
                        - body.position(self))
                    .normalize();
                    force += force_dir
                        * UNIVERSAL_GRAVITY
                        * GRAVITY_AMPLIFIER
                        * body.mass(self)
                        * other.mass(self)
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
