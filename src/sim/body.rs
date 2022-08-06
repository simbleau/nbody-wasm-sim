use nalgebra::Normed;
use rapier2d::prelude::*;

use super::simulation::Simulation;

pub struct Body {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl Body {
    pub fn mass(&self, sim: &Simulation) -> f32 {
        sim.rigid_body_set
            .get(self.rigid_body_handle)
            .unwrap()
            .mass()
    }

    pub fn radius(&self, sim: &Simulation) -> f32 {
        sim.collider_set
            .get(self.collider_handle)
            .unwrap()
            .shape()
            .as_ball()
            .unwrap()
            .radius
    }

    pub fn rotation(&self, sim: &Simulation) -> f32 {
        sim.rigid_body_set
            .get(self.rigid_body_handle)
            .unwrap()
            .rotation()
            .angle()
    }

    pub fn x(&self, sim: &Simulation) -> f32 {
        sim.rigid_body_set
            .get(self.rigid_body_handle)
            .unwrap()
            .position()
            .translation
            .x
    }

    pub fn y(&self, sim: &Simulation) -> f32 {
        sim.rigid_body_set
            .get(self.rigid_body_handle)
            .unwrap()
            .position()
            .translation
            .y
    }
}
