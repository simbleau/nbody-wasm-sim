use glam::{Mat4, Quat, Vec2};

pub struct Camera {
    scale: f32,
    rotation: f32,
    translation: Vec2,
    view_size: Vec2,
}

impl Camera {
    pub fn new(
        view_size: Vec2,
        rotation: f32,
        translation: Vec2,
        scale: f32,
    ) -> Self {
        Camera {
            scale,
            rotation,
            translation,
            view_size,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::from_rotation_translation(
            Quat::from_rotation_z(self.rotation),
            self.translation.extend(1.0),
        );

        let (width, height) = self.view_size.into();
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        let left = -half_width;
        let right = half_width;
        let top = half_height;
        let bottom = -half_height;

        let proj = Mat4::orthographic_rh(
            left / self.scale,
            right / self.scale,
            bottom / self.scale,
            top / self.scale,
            0.0,
            1.0,
        );

        return proj * view.inverse();
    }
}
