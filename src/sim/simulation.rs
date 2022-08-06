use std::ops::Mul;

use glam::{Mat3, Quat, Vec2, Vec3, Vec3Swizzles};
use instant::Instant;
use rapier2d::prelude::*;
use winit::event::VirtualKeyCode;

use crate::sim::{State, WORLD_RADIUS};

use super::physics::PhysicsContext;

pub const CAM_ZOOM_SPEED: f32 = 5.0;
pub const CAM_ROTATE_SPEED: f32 = 5.0;
pub const CAM_PAN_SPEED: f32 = 400.0;
pub const DAMPENING: f32 = 0.05;

pub const RESTITUTION: f32 = 0.8;
pub const FRICTION: f32 = 0.8;
pub const PIXEL_DISTANCE: f32 = 100_000_000_000.0; // Meters
pub const UNIVERSAL_GRAVITY: f32 = 0.000000000066743 * PIXEL_DISTANCE;

pub struct Simulation {
    pub state: State,
    pub physics_context: PhysicsContext,
}

impl Simulation {
    pub fn new(num_bodies: usize) -> Self {
        // Generate a bunch of bodies
        let radius_max = 1.0;
        let angvel_max = 0.1 * (2.0 * std::f64::consts::PI);
        let linvel_max = 2.0;
        let rngify = |x| (js_sys::Math::random() * x) as f32;

        let mut physics_context = PhysicsContext::new();
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

            // Bodies
            let rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                .translation(vector![position.x, position.y])
                .linvel(vector![linvel.x, linvel.y])
                .angvel(angvel)
                .ccd_enabled(true)
                .rotation(rotation)
                .build();
            let collider = ColliderBuilder::ball(radius)
                .restitution(RESTITUTION)
                .friction(FRICTION)
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
            state.texture_key = match &state.texture_key as &str {
                "moon" => "cookie".to_owned(),
                _ => "moon".to_owned(),
            };
        }

        state.pan += state.pan_velocity * dt;
    }
}
