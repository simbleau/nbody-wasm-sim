use nalgebra::Vector2;

const WAVE_SPEED: f32 = 3.14 * 2.0; // 2 oscillations per second

#[derive(Clone, Debug)]
pub struct Body {
    pub verts: [Vector2<f32>; 3],
    pub elapsed: f32,
}

impl Default for Body {
    fn default() -> Self {
        Body {
            verts: [
                Vector2::new(0.0, 0.5), // This one gets moved (verts[0])
                Vector2::new(-0.5, -0.5),
                Vector2::new(0.5, -0.5),
            ],
            elapsed: 0.0,
        }
    }
}

impl Body {
    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;

        // Make the top vertex move in a circle
        self.verts[0].x = (self.elapsed * WAVE_SPEED).cos() / 3.0 + 0.0;
        self.verts[0].y = (self.elapsed * WAVE_SPEED).sin() / 3.0 + 0.25;
    }
}
