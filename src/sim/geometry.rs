use nalgebra::Vector2;

#[derive(Clone, Debug)]
pub enum Geometry {
    Circle(f32),
    Triangle([Vector2<f32>;3]),
}