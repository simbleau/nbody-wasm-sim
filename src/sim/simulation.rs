use glam::{Mat3, Quat, Vec2, Vec3, Vec3Swizzles};
use instant::Instant;
use rapier2d::prelude::*;
use std::ops::Mul;
use winit::event::VirtualKeyCode;

use crate::sim::{Body, State, WORLD_RADIUS};

use super::physics::get_collision;

pub const CAM_ZOOM_SPEED: f32 = 5.0;
pub const CAM_ROTATE_SPEED: f32 = 5.0;
pub const CAM_PAN_SPEED: f32 = 400.0;
pub const DAMPENING: f32 = 0.05;


pub const RESTITUTION: f32 = 0.8;
pub const FRICTION: f32 = 0.8;
pub const PIXEL_DISTANCE: f32 = 100_000_000_000.0; // Meters
pub const UNIVERSAL_GRAVITY: f32 = 0.000000000066743 * PIXEL_DISTANCE;

pub struct Simulation<'a> {
    pub state: State<'a>,
    pub bodies: Vec<Body>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    ccd_solver: CCDSolver,
    pub(crate) rigid_body_set: RigidBodySet,
    pub(crate) collider_set: ColliderSet,
}

impl<'a> Simulation<'a> {
    pub fn new(num_bodies: u32, state: State<'a>) -> Self {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        // Generate a bunch of bodies
        let radius_max = 1.0;
        let angvel_max = 0.0 * (2.0 * std::f64::consts::PI);
        let linvel_max = 3.0;
        let rngify = |x| (js_sys::Math::random() * x) as f32;
        let mut bodies = vec![];
        for _ in 0..num_bodies {
            // Calculate radius
            let rotation = rngify(2.0 * std::f64::consts::PI);
            let radius = rngify(radius_max);

            // Calculate initial world position as polar coordinates
            let r = WORLD_RADIUS * rngify(1.0).sqrt() - radius;
            let theta = rngify(2.0 * std::f64::consts::PI);
            let position = Mat3::from_rotation_z(theta) * Vec3::X.mul(r);

            // Calculate initial velocity
            let linvel_theta = rngify(2.0 * std::f64::consts::PI);
            let magnitude = rngify(linvel_max);
            let linvel =
                Mat3::from_rotation_z(linvel_theta) * Vec3::X.mul(magnitude);

            // Calculate initial angular velocity
            let angvel = rngify(angvel_max);

            // Create physics components
            let rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                .translation(vector![position.x, position.y])
                .linvel(vector![linvel.x, linvel.y])
                .angvel(angvel)
                .ccd_enabled(false)
                .rotation(rotation)
                .build();
            let collider = ColliderBuilder::ball(radius)
                .restitution(RESTITUTION)
                .friction(FRICTION)
                .build();
            let rigid_body_handle = rigid_body_set.insert(rigid_body);
            let collider_handle = collider_set.insert_with_parent(
                collider,
                rigid_body_handle,
                &mut rigid_body_set,
            );

            let body = Body {
                rigid_body_handle,
                collider_handle,
            };
            bodies.push(body);

        /* Create other structures necessary for the simulation. */
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let ccd_solver = CCDSolver::new();

        Self {
            state,
            bodies,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            ccd_solver,
            rigid_body_set,
            collider_set,
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.integration_parameters.dt = dt;

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

        // Calculate velocity vectors
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

    pub fn update(&mut self) {
        // Pausing
        if self
            .state
            .input_controller
            .is_key_pressed(VirtualKeyCode::Space)
        {
            self.state.paused = !self.state.paused;
        }

        // Get delta time
        let now = Instant::now();
        let dt = (now - self.state.last_frame.unwrap_or(now)).as_secs_f32();
        self.state.last_frame.replace(now);

        if !self.state.paused {
            self.step(dt);
        }
        self.update_camera(dt);

        // Reset input controller
        self.state.input_controller.update();
    }

    pub fn update_camera(&mut self, dt: f32) {
        // Handle input
        let state = &mut self.state;
        // Rotation
        if state.input_controller.is_key_active(VirtualKeyCode::Left) {
            state.rotation += CAM_ROTATE_SPEED * dt;
        }
        if state.input_controller.is_key_active(VirtualKeyCode::Right) {
            state.rotation -= CAM_ROTATE_SPEED * dt;
        }
        // Scale
        if state.input_controller.is_key_active(VirtualKeyCode::Up) {
            state.zoom += state.zoom * CAM_ZOOM_SPEED * dt;
        }
        if state.input_controller.is_key_active(VirtualKeyCode::Down) {
            state.zoom -= state.zoom * CAM_ZOOM_SPEED * dt;
        }
        // Translation
        let mut cam_direction = Vec2::ZERO;
        if state.input_controller.is_key_active(VirtualKeyCode::W) {
            cam_direction +=
                (Quat::from_rotation_z(state.rotation) * (Vec3::Y)).xy();
        }
        if state.input_controller.is_key_active(VirtualKeyCode::A) {
            cam_direction -=
                (Quat::from_rotation_z(state.rotation) * (Vec3::X)).xy();
        }
        if state.input_controller.is_key_active(VirtualKeyCode::S) {
            cam_direction -=
                (Quat::from_rotation_z(state.rotation) * (Vec3::Y)).xy();
        }
        if state.input_controller.is_key_active(VirtualKeyCode::D) {
            cam_direction +=
                (Quat::from_rotation_z(state.rotation) * (Vec3::X)).xy();
        }

        // Normalize
        cam_direction = cam_direction.normalize_or_zero();

        // Camera movement
        if state.input_controller.is_one_of_key_active(vec![
            VirtualKeyCode::W,
            VirtualKeyCode::A,
            VirtualKeyCode::S,
            VirtualKeyCode::D,
        ]) {
            // Move camera
            state.pan_velocity = (cam_direction * CAM_PAN_SPEED) / state.zoom;
        } else if state.pan_velocity.length_squared() > 0.0 {
            // Dampen camera velocity
            state.pan_velocity += -1.0 * state.pan_velocity * DAMPENING;
        }
        // Wireframe
        if state.input_controller.is_key_pressed(VirtualKeyCode::Q) {
            state.wireframe = !state.wireframe;
        }
        // Texture Change
        if state.input_controller.is_key_released(VirtualKeyCode::E) {
            state.texture_key = match state.texture_key {
                "moon" => "cookie",
                _ => "moon",
            };
        }

        state.pan += state.pan_velocity * dt;
    }
}
