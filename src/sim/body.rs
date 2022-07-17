use nalgebra::Vector2;

const WAVE_SPEED: f32 = 3.14 * 2.0;

#[derive(Clone, Debug)]
pub struct Body {
    pub origin: Vector2<f32>,
    pub geometry: Geometry,
    pub elapsed: f32,
}

impl Default for Body {
    fn default() -> Self {
        Body {
            origin: Vector2::new(0.0, 0.0),
            geometry: Geometry::Triangle([
                Vector2::new(0.0, 0.5),
                Vector2::new(-0.5, -0.5),
                Vector2::new(0.5, -0.5),
            ]),
            elapsed: 0.0,
        }
    }
}

impl Body {
    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;

        // Make the top vertex move in a circle
        self.origin.x = (self.elapsed * WAVE_SPEED).cos() / 3.0;
        self.origin.y = (self.elapsed * WAVE_SPEED).sin() / 3.0;
    }
}

use crate::gpu_primitives::{GpuCircle, GpuTriangle, GpuVertex};

use super::geometry::Geometry;

impl From<&Body> for GpuTriangle {
    fn from(body: &Body) -> Self {
        match body.geometry {
            Geometry::Triangle(verts) => {
                let p1_x = body.origin.x + verts[0].x;
                let p1_y = body.origin.y + verts[0].y;

                let p2_x = body.origin.x + verts[1].x;
                let p2_y = body.origin.y + verts[1].y;

                let p3_x = body.origin.x + verts[2].x;
                let p3_y = body.origin.y + verts[2].y;

                let verts = [
                    GpuVertex {
                        position: [p1_x, p1_y, 0.0],
                        color: [1.0, 0.0, 0.0],
                    },
                    GpuVertex {
                        position: [p2_x, p2_y, 0.0],
                        color: [0.0, 1.0, 0.0],
                    },
                    GpuVertex {
                        position: [p3_x, p3_y, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                ];
                GpuTriangle { verts }
            }
            Geometry::Circle(_) => {
                unimplemented!();
            }
        }
    }
}

impl From<&Body> for GpuCircle {
    fn from(body: &Body) -> Self {
        match body.geometry {
            Geometry::Triangle(_) => {
                unimplemented!();
            }
            Geometry::Circle(radius) => GpuCircle {
                position: [body.origin.x, body.origin.y],
                radius,
            },
        }
    }
}
