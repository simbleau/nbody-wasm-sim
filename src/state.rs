use instant::Instant;
use nalgebra::Vector2;
use winit::event::WindowEvent;

use crate::{
    body::Body,
    frame_description::{FrameDescription, GpuTriangle, Vertex},
};

pub struct State {
    pub mouse_pos: Vector2<f64>,
    pub window_size: Vector2<u32>,
    pub last_frame: Option<Instant>,
    pub bodies: Vec<Body>,
    pub wireframe: bool,
    pub paused: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mouse_pos: Vector2::default(),
            window_size: Vector2::default(),
            last_frame: None,
            bodies: vec![Body {
                verts: [
                    Vector2::new(0.0, 0.5), // This one gets moved (verts[0])
                    Vector2::new(-0.5, -0.5),
                    Vector2::new(0.5, -0.5),
                ],
                elapsed: 0.0,
            }],
            wireframe: false,
            paused: false,
        }
    }
}

impl State {
    pub fn input(&mut self, event: &WindowEvent) {
        // We have no events to handle currently
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = Vector2::new(position.x, position.y);
            }
            WindowEvent::Resized(size) => {
                self.window_size = Vector2::new(size.width, size.height);
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        // Remain paused
        if self.paused {
            self.last_frame = Some(Instant::now());
            return;
        }

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

    pub fn get_frame_desc(&self) -> FrameDescription {
        let mut gpu_triangles = Vec::new();

        for body in self.bodies.iter() {
            let verts = [
                Vertex {
                    position: [body.verts[0].x, body.verts[0].y, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [body.verts[1].x, body.verts[1].y, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [body.verts[2].x, body.verts[2].y, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
            ];
            gpu_triangles.push(GpuTriangle { verts })
        }

        FrameDescription { gpu_triangles }
    }
}
