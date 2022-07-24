use glam::{DVec2, DVec3, Quat, UVec2, Vec2, Vec3, Vec3Swizzles};
use instant::Instant;
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

use crate::sim::{geometry::Geometry, Body};

pub const INITIAL_VIEW_BOUNDS: Vec2 = Vec2::new(3., 3.);
pub const CAM_PAN_SPEED: f32 = 0.05;

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
    pub rotation: f32,
    pub zoom: f32,
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
            rotation: 0.0,
            zoom: 100.0,
        }
    }
}

impl<'a> State<'a> {
    pub fn new(view_size: Vec2) -> Self {
        // Generate a bunch of bodies
        let triangle_size_max = 1.0;
        let displacement_max = 3.0;
        let half_displacement_max = displacement_max / 2.0;
        let rngify = |x| (js_sys::Math::random() * x) as f32;
        let mut bodies = Vec::new();
        for _ in 0..1 {
            let mut body = Body::new(Geometry::Triangle([
                Vec2::new(0.0, rngify(triangle_size_max)),
                Vec2::new(
                    -rngify(triangle_size_max),
                    -rngify(triangle_size_max),
                ),
                Vec2::new(
                    rngify(triangle_size_max),
                    -rngify(triangle_size_max),
                ),
            ]));
            body.origin.x +=
                rngify(displacement_max) - half_displacement_max as f32;
            body.origin.y +=
                rngify(displacement_max) - half_displacement_max as f32;
            bodies.push(body);
        }

        Self {
            pan: Vec2::new(0., 0.),
            zoom: if (view_size.y - INITIAL_VIEW_BOUNDS.y).abs()
                < (view_size.x / INITIAL_VIEW_BOUNDS.x).abs()
            {
                view_size.y / INITIAL_VIEW_BOUNDS.y
            } else {
                view_size.x / INITIAL_VIEW_BOUNDS.x
            },
            bodies,
            ..Default::default()
        }
    }

    pub fn input(&mut self, event: &WindowEvent) {
        // We have no events to handle currently
        match event {
            // Rotation
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Left) =>
            {
                self.rotation += 0.1;
            }
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Right) =>
            {
                self.rotation -= 0.1;
            }
            // Scale
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Up) =>
            {
                self.zoom += self.zoom * 0.1;
            }
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Down) =>
            {
                self.zoom -= self.zoom * 0.1;
            }
            // Translation
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::W) =>
            {
                self.pan += (Quat::from_rotation_z(self.rotation)
                    * (CAM_PAN_SPEED * Vec3::Y))
                    .xy();
            }
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::A) =>
            {
                self.pan -= (Quat::from_rotation_z(self.rotation)
                    * (CAM_PAN_SPEED * Vec3::X))
                    .xy();
            }
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::S) =>
            {
                self.pan -= (Quat::from_rotation_z(self.rotation)
                    * (CAM_PAN_SPEED * Vec3::Y))
                    .xy();
            }
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::D) =>
            {
                self.pan += (Quat::from_rotation_z(self.rotation)
                    * (CAM_PAN_SPEED * Vec3::X))
                    .xy();
            }
            // Change sim visuals
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Space)
                    && input.state == ElementState::Released =>
            {
                self.paused = !self.paused;
            }
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Q)
                    && input.state == ElementState::Released =>
            {
                self.wireframe = !self.wireframe;
            }
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::E)
                    && input.state == ElementState::Released =>
            {
                self.texture_key = match self.texture_key {
                    "moon" => "cookie",
                    _ => "moon",
                };
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
        // Remain paused
        if self.paused {
            self.last_frame = Some(Instant::now());
            self.bg_color = DVec3::default();
            return;
        }

        self.bg_color = DVec3::new(0., 1.0, 0.);
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
            }
            None => {
                self.last_frame = Some(Instant::now());
            }
        }
    }
}
