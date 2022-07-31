#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WorldUniform {
    pub radius: f32,
    pub boundary_segments: u32,
    pub padding: [f32; 2],
}

unsafe impl bytemuck::Pod for WorldUniform {}
unsafe impl bytemuck::Zeroable for WorldUniform {}
