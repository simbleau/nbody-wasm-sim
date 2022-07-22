#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

unsafe impl bytemuck::Pod for CameraUniform {}
unsafe impl bytemuck::Zeroable for CameraUniform {}
