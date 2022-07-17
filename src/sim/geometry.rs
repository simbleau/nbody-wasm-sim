use glam::Vec2;

#[derive(Clone, Debug)]
pub enum Geometry {
    Circle(f32),
    Triangle([Vec2; 3]),
}
