use glam::{DVec2, DVec3, Mat3, Quat, UVec2, Vec2, Vec3, Vec3Swizzles};
use instant::Instant;
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

use crate::sim::Body;

use super::input::InputController;

pub const INITIAL_VIEW_BOUNDS: f32 = 100.0;
pub const CAM_ZOOM_SPEED: f32 = 20.0;
pub const CAM_ROTATE_SPEED: f32 = 5.0;
pub const CAM_MAX_PAN_SPEED: f32 = 100.0;
pub const CAM_PAN_ACCELERATION: f32 = 0.1;

pub struct State<'a> {
    pub mouse_pos: DVec2,
    pub view_size: UVec2,
    pub last_frame: Option<Instant>,
    pub bodies: Vec<Body>,
    pub wireframe: bool,
    pub paused: bool,
    pub bg_color: DVec3,
    pub texture_key: &'a str,
    pub pan: Vec2,
    pub pan_velocity: Vec2,
    pub rotation: f32,
    pub zoom: f32,
    pub input_controller: InputController,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self {
            mouse_pos: DVec2::default(),
            view_size: UVec2::default(),
            last_frame: None,
            bodies: vec![],
            wireframe: false,
            paused: false,
            bg_color: DVec3::default(),
            texture_key: "moon",
            pan: Vec2::ZERO,
            pan_velocity: Vec2::ZERO,
            rotation: 0.0,
            zoom: 100.0,
            input_controller: InputController::default(),
        }
    }
}

impl<'a> State<'a> {
    pub fn new(view_size: Vec2) -> Self {
        let zoom = if view_size.y < view_size.x {
            view_size.y / INITIAL_VIEW_BOUNDS
        } else {
            view_size.x / INITIAL_VIEW_BOUNDS
        };

        // Generate a bunch of bodies
        let radius_max = 1.0;
        let rngify = |x| (js_sys::Math::random() * x) as f32;
        let mut bodies = Vec::new();
        for _ in 0..1000 {
            let mut body = Body::new(Vec2::ZERO, 0.0, rngify(radius_max));

            let r = INITIAL_VIEW_BOUNDS / 2.0 * rngify(1.0).sqrt();
            let displacement = Vec3::new(r, 0.0, 0.0);
            let direction =
                Mat3::from_rotation_z(rngify(std::f64::consts::PI * 2.0));

            body.origin = (direction * displacement).xy();

            bodies.push(body);
        }

        Self {
            pan: Vec2::new(0., 0.),
            zoom,
            bodies,
            ..Default::default()
        }
    }

    pub fn handle_input(&mut self, event: &WindowEvent) {
        // We have no events to handle currently
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(key) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            self.input_controller.press(key)
                        }
                        ElementState::Released => {
                            self.input_controller.release(key)
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = DVec2::new(position.x, position.y);
            }
            WindowEvent::Resized(size) => {
                self.view_size = UVec2::new(size.width, size.height);
            }
            _ => {}
        }
    }

    fn update_camera(&mut self, dt: f32) {
        // Handle input
        // Rotation
        if self.input_controller.is_key_active(VirtualKeyCode::Left) {
            self.rotation += CAM_ROTATE_SPEED * dt;
        }
        if self.input_controller.is_key_active(VirtualKeyCode::Right) {
            self.rotation -= CAM_ROTATE_SPEED * dt;
        }
        // Scale
        if self.input_controller.is_key_active(VirtualKeyCode::Up) {
            self.zoom += self.zoom * CAM_ZOOM_SPEED * dt;
        }
        if self.input_controller.is_key_active(VirtualKeyCode::Down) {
            self.zoom -= self.zoom * CAM_ZOOM_SPEED * dt;
        }
        // Translation
        let mut acceleration_direction = Vec2::ZERO;
        if self.input_controller.is_key_active(VirtualKeyCode::W) {
            acceleration_direction +=
                (Quat::from_rotation_z(self.rotation) * (Vec3::Y)).xy();
        }
        if self.input_controller.is_key_active(VirtualKeyCode::A) {
            acceleration_direction -=
                (Quat::from_rotation_z(self.rotation) * (Vec3::X)).xy();
        }
        if self.input_controller.is_key_active(VirtualKeyCode::S) {
            acceleration_direction -=
                (Quat::from_rotation_z(self.rotation) * (Vec3::Y)).xy();
        }
        if self.input_controller.is_key_active(VirtualKeyCode::D) {
            acceleration_direction +=
                (Quat::from_rotation_z(self.rotation) * (Vec3::X)).xy();
        }

        // Normalize
        acceleration_direction = acceleration_direction.normalize_or_zero();

        // Camera acceleration
        if self.input_controller.is_one_of_key_active(vec![
            VirtualKeyCode::W,
            VirtualKeyCode::A,
            VirtualKeyCode::S,
            VirtualKeyCode::D,
        ]) {
            // Acceleration
            self.pan_velocity = (self.pan_velocity
                + acceleration_direction * CAM_PAN_ACCELERATION)
                .clamp(
                    Vec2::splat(-CAM_MAX_PAN_SPEED),
                    Vec2::splat(CAM_MAX_PAN_SPEED),
                );
        } else if self.pan_velocity.length_squared() > 0.0 {
            // Dampening
            let deceleration_direction = -1.0 * self.pan_velocity.normalize();

            let delta_v = deceleration_direction * CAM_PAN_ACCELERATION;

            let new_velocity = self.pan_velocity + delta_v;

            if new_velocity.dot(self.pan_velocity) < 0.0 {
                self.pan_velocity = Vec2::ZERO;
            } else {
                self.pan_velocity = new_velocity;
            }
        }
        // Wireframe
        if self.input_controller.is_key_pressed(VirtualKeyCode::Q) {
            self.wireframe = !self.wireframe;
        }
        // Texture Change
        if self.input_controller.is_key_released(VirtualKeyCode::E) {
            self.texture_key = match self.texture_key {
                "moon" => "cookie",
                _ => "moon",
            };
        }

        self.pan += self.pan_velocity * dt / self.zoom;
    }

    pub fn update(&mut self) {
        // Pausing
        if self.input_controller.is_key_pressed(VirtualKeyCode::Space) {
            self.paused = !self.paused;
        }

        // Remain paused
        if self.paused {
            self.last_frame = Some(Instant::now());
            return;
        }

        // Update sim
        match self.last_frame {
            Some(last_frame) => {
                let now = Instant::now();
                let dt = now - last_frame;

                // Simulation logic
                let dt_f32 = dt.as_secs_f32();
                for body in self.bodies.iter_mut() {
                    body.update(dt_f32);
                }
                self.last_frame = Some(now);

                // Handle camera input
                self.update_camera(dt_f32);
            }
            None => {
                self.last_frame = Some(Instant::now());
            }
        }

        // Reset input controller
        self.input_controller.update();
    }
}
