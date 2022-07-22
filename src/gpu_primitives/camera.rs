use glam::Mat4;

use crate::render::Camera;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj =
            camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

unsafe impl bytemuck::Pod for CameraUniform {}
unsafe impl bytemuck::Zeroable for CameraUniform {}
