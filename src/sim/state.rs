use glam::{DVec2, DVec3, Mat3, UVec2, Vec2, Vec3, Vec3Swizzles};
use instant::Instant;
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

use crate::sim::{input::InputController, simulation, Body, WORLD_RADIUS};

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
            view_size.y / (WORLD_RADIUS * 2.0)
        } else {
            view_size.x / (WORLD_RADIUS * 2.0)
        };

        // Generate a bunch of bodies
        let radius_max = 1.0;
        let rngify = |x| (js_sys::Math::random() * x) as f32;
        let mut bodies = Vec::new();
        for _ in 0..1000 {
            let body_radius = rngify(radius_max);
            let mut body = Body::new(Vec2::ZERO, 0.0, body_radius);

            let r = WORLD_RADIUS * rngify(1.0).sqrt() - body_radius;
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

    pub fn update(&mut self) {
        // Pausing
        if self.input_controller.is_key_pressed(VirtualKeyCode::Space) {
            self.paused = !self.paused;
        }

        // Get delta time
        let now = Instant::now();
        let dt = (now - self.last_frame.unwrap_or(now)).as_secs_f32();
        self.last_frame.replace(now);

        if self.paused {
            // Only update camera
            simulation::update_camera(self, dt);
        } else {
            // Update simulation
            simulation::update(self, dt);
        }

        // Reset input controller
        self.input_controller.update();
    }
}
