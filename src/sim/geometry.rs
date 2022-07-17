use glam::Vec2;

#[derive(Clone, Debug)]
pub enum Geometry {
    #[allow(dead_code)]
    Circle(f32),
    Triangle([Vec2; 3]),
}
