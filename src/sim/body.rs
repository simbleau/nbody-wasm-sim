use glam::Vec2;

#[derive(Clone, Debug)]
pub struct Body {
    pub origin: Vec2,
    pub geometry: Geometry,
    pub elapsed: f32,
}

impl Default for Body {
    fn default() -> Self {
        Body {
            origin: Vec2::new(0.0, 0.0),
            geometry: Geometry::Triangle([
                Vec2::new(0.0, 0.5),
                Vec2::new(-0.5, -0.5),
                Vec2::new(0.5, -0.5),
            ]),
            elapsed: 0.0,
        }
    }
}

impl Body {
    pub fn new(shape: Geometry) -> Self {
        Body {
            geometry: shape,
            ..Default::default()
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
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
                        uv: [0.5, 0.25],
                    },
                    GpuVertex {
                        position: [p2_x, p2_y, 0.0],
                        uv: [0.25, 0.75],
                    },
                    GpuVertex {
                        position: [p3_x, p3_y, 0.0],
                        uv: [0.75, 0.75],
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
