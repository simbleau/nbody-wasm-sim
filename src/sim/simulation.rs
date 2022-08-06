use glam::{Quat, Vec2, Vec3, Vec3Swizzles};
use winit::event::VirtualKeyCode;

use crate::sim::State;

use super::physics::get_collision;

pub const CAM_ZOOM_SPEED: f32 = 5.0;
pub const CAM_ROTATE_SPEED: f32 = 5.0;
pub const CAM_PAN_SPEED: f32 = 400.0;
pub const DAMPENING: f32 = 0.05;

pub const TIME_STEP: f32 = 500_000.0; // Seconds
pub const PIXEL_DISTANCE: f32 = 500_000_000_000.0; // Meters
pub const SPEED_OF_LIGHT: f32 = 299_792_458.0 / PIXEL_DISTANCE;
pub const UNIVERSAL_GRAVITY: f32 = 0.000000000066743 * PIXEL_DISTANCE;
pub const TERMINAL_VELOCITY: Vec2 =
    Vec2::new(SPEED_OF_LIGHT * TIME_STEP, SPEED_OF_LIGHT * TIME_STEP);

pub fn update(state: &mut State, dt: f32) {
    // Calculate velocity vectors
    let num_bodies = state.bodies.len();
    for i in 0..num_bodies {
        // Get displacement
        let body = &state.bodies[i];
        let mut velocity = body.velocity;
        for other in &state.bodies {
            if body != other {
                let sqr_dist =
                    (other.position - body.position).length_squared();
                let force_dir = (other.position - body.position).normalize();
                let force =
                    force_dir * UNIVERSAL_GRAVITY * body.mass() / sqr_dist;
                let acceleration = force / body.mass();
                velocity += acceleration * dt;
            }
        }
        // Adjust body
        *&mut state.bodies[i].velocity =
            velocity.clamp(-TERMINAL_VELOCITY, TERMINAL_VELOCITY);
    }

    'A: for i in 0..num_bodies {
        let body = &state.bodies[i];
        for j in i + 1..num_bodies {
            let other = &state.bodies[j];
            if let Some(collision) = get_collision(body, other) {
                *&mut state.bodies[i].velocity = collision.v1_final;
                *&mut state.bodies[j].velocity = collision.v2_final;

                continue 'A;
            }
        }
    }

    // Update position
    for body in state.bodies.iter_mut() {
        body.position += body.velocity * dt;
    }

    // Handle camera input
    update_camera(state, dt);
}

pub fn update_camera(state: &mut State, dt: f32) {
    // Handle input
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
