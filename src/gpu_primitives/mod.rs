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

mod world_boundaries;
pub use world_boundaries::WorldBoundaryUniform;

mod world_scale;
pub use world_scale::WorldRadiusUniform;
