use super::WgpuContext;
use wgpu::RenderPipeline;

mod solid;
mod wireframe;

pub enum Pipeline {
    Wireframe,
    Solid,
}

impl Pipeline {
    pub fn get(&self, context: &WgpuContext) -> RenderPipeline {
        match self {
            Pipeline::Wireframe => wireframe::get(context),
            Pipeline::Solid => solid::get(context),
        }
    }
}
