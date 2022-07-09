use crate::gpu_primitives::GpuTriangle;

pub struct FrameDescription {
    pub gpu_triangles: Vec<GpuTriangle>,
}

impl FrameDescription {
    pub fn get_vertex_buffer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        for instance in self.gpu_triangles.iter() {
            let bytes = bytemuck::cast_slice(&instance.verts);
            buf.extend(bytes);
        }

        buf
    }
}
