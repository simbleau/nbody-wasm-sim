pub trait GpuPrimitive {
    fn data(&self) -> Vec<u8>;
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

mod vertex;
pub use vertex::GpuVertex;

mod triangle;
pub use triangle::GpuTriangle;

mod circle;
pub use circle::GpuCircle;

mod camera;
pub use camera::CameraUniform;

mod instance;
pub use instance::InstanceRaw;
