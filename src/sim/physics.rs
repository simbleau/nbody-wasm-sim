use particular::ParticleSet;
use rapier2d::prelude::*;

use super::body::Body;

pub struct PhysicsContext {
    pub bodies: ParticleSet<Body>,
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
            bodies: ParticleSet::<Body>::new(),
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

        self.bodies.add(particle);
    }

    pub fn step(&mut self) {
        for (body, acceleration) in self.bodies.result() {
            body.apply_acceleration_to_rigidbody(
                &mut self.rigid_body_set,
                acceleration,
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

        for body in self.bodies.iter_mut() {
            body.sync_to_rigidbody(&self.rigid_body_set, &self.collider_set);
        }
    }
}
