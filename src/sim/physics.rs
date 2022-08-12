use rapier2d::prelude::*;

use super::body::{Body, BodySet};

pub struct PhysicsContext {
    pub bodies: BodySet,
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
            bodies: BodySet::new(),
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

        let mut particle = Body::new(rigid_body_handle, collider_handle);
        particle.sync_to_rigidbody(&self.rigid_body_set, &self.collider_set);

        self.bodies.particles.push(particle);
    }

    pub fn step(&mut self) {
        self.bodies.response(|mut result| {
            for (particle, acceleration) in &result {
                particle.apply_force_to_rigidbody(
                    &mut self.rigid_body_set,
                    *acceleration * particle.mass(),
                )
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

            for (particle, _) in &mut result {
                particle.sync_to_rigidbody(
                    &self.rigid_body_set,
                    &self.collider_set,
                );
            }
        });
    }
}
