use glam::Vec2;

#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    pub position: Vec2,
    pub radius: f32,
    pub rotation: f32,
    pub elapsed: f32,
    pub density: f32,
    pub velocity: Vec2,
}

impl Default for Body {
    fn default() -> Self {
        Body {
            position: Vec2::new(0.0, 0.0),
            radius: 0.5,
            elapsed: 0.0,
            rotation: 0.0,
            velocity: Vec2::ZERO,
            density: 1.0,
        }
    }
}

impl Body {
    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    pub fn mass(&self) -> f32 {
        self.area() * self.density
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
    }

    pub fn new(origin: Vec2, rotation: f32, radius: f32) -> Self {
        Self {
            position: origin,
            radius,
            rotation,
            ..Default::default()
        }
    }
}
