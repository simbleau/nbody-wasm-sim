pub trait GpuPrimitive {
    fn data(&self) -> Vec<u8>;
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

mod vertex;
pub use vertex::GpuVertex;

mod quad;
pub use quad::GpuQuad;

mod camera;
pub use camera::CameraUniform;

mod transform;
pub use transform::GpuTransform;
