use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, VertexBufferLayout};

pub trait GpuPrimitive {
    fn data(&self) -> Vec<u8>;
    fn desc<'a>() -> VertexBufferLayout<'a>;
}

pub trait GpuUniform {
    fn bind(
        &self,
        device: &Device,
    ) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout);
}

mod vertex;
pub use vertex::GpuVertex;

mod quad;
pub use quad::GpuQuad;

mod transform;
pub use transform::GpuTransform;

mod camera;
pub use camera::CameraUniform;

mod world;
pub use world::WorldUniform;
