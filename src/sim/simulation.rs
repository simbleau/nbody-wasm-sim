use std::ops::Mul;

use glam::{Mat3, Quat, Vec2, Vec3, Vec3Swizzles};
use instant::Instant;
use rapier2d::prelude::*;
use winit::event::VirtualKeyCode;

use crate::sim::physics::PhysicsContext;
use crate::sim::State;

// Universe
pub const UNIVERSAL_GRAVITY: f32 = 0.000000000066743;
pub const GRAVITY_AMPLIFIER: f32 = 40_000_000_000.0;
pub const WORLD_RADIUS: f32 = 75.0;
pub const WORLD_EDGE_SEGMENTS: u32 = 500;

// Camera
pub const CAM_ZOOM_SPEED: f32 = 5.0;
pub const CAM_ROTATE_SPEED: f32 = 5.0;
pub const CAM_PAN_SPEED: f32 = 400.0;
pub const DAMPENING: f32 = 0.05;

// Bodies
pub const DEFAULT_NUM_BODIES: usize = 100;
pub const BODY_MAX_RADIUS: f64 = 1.0;
pub const BODY_MAX_ANG_VEL: f64 = 0.2 * (2.0 * std::f64::consts::PI);
pub const BODY_MAX_LIN_VEL: f64 = 3.0;
pub const BODY_RESTITUTION: f32 = 0.6;
pub const BODY_FRICTION: f32 = 0.9;

pub struct Simulation {
    pub state: State,
    pub physics_context: PhysicsContext,
}

impl Default for Simulation {
    fn default() -> Self {
        Simulation::new(DEFAULT_NUM_BODIES)
    }
}

impl Simulation {
    pub fn new(num_bodies: usize) -> Self {
        // Generate a bunch of bodies
        let rngify = |x| (js_sys::Math::random() * x) as f32;

        let mut physics_context = PhysicsContext::new();
        for _ in 0..num_bodies {
            // Calculate radius
            let rotation = rngify(2.0 * std::f64::consts::PI);
            let radius = rngify(BODY_MAX_RADIUS.sqrt());

            // Calculate initial world position as polar coordinates
            let r = WORLD_RADIUS * rngify(1.0).sqrt() - radius;
            let theta = rngify(2.0 * std::f64::consts::PI);
            let position = Mat3::from_rotation_z(theta) * Vec3::X.mul(r);

            // Calculate initial velocity
            let linvel_theta = rngify(2.0 * std::f64::consts::PI);
            let magnitude = rngify(BODY_MAX_LIN_VEL);
            let linvel =
                Mat3::from_rotation_z(linvel_theta) * Vec3::X.mul(magnitude);

            // Calculate initial angular velocity
            let angvel = rngify(BODY_MAX_ANG_VEL);

            // Bodies
            let rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                .translation(vector![position.x, position.y])
                .linvel(vector![linvel.x, linvel.y])
                .angvel(angvel)
                .rotation(rotation)
                .ccd_enabled(false) // Enable for higher accuracy
                .build();
            let collider = ColliderBuilder::ball(radius)
                .restitution(BODY_RESTITUTION)
                .friction(BODY_FRICTION)
                .build();
            physics_context.create_body(rigid_body, collider);
        }

        Self {
            state: State::default(),
            physics_context,
        }
    }

    pub fn update(&mut self) {
        // Check for pause key
        if self
            .state
            .input_controller
            .is_key_pressed(VirtualKeyCode::Space)
        {
            self.state.paused = !self.state.paused;
        }

        // Step simulation
        if !self.state.paused {
            self.physics_context.step();
        }

        // Update last frame, get delta time
        let now = Instant::now();
        let dt = (now - self.state.last_frame.unwrap_or(now)).as_secs_f32();
        gloo_console::log!("frame-time: ", dt);
        self.state.last_frame.replace(now);

        // Control camera
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
            state.rave = !state.rave;
            state.texture_key = match &state.texture_key as &str {
                "rust" => "disco".to_owned(),
                _ => "rust".to_owned(),
            };
        }

        state.pan += state.pan_velocity * dt;
    }
}
