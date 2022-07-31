use super::WgpuContext;
use wgpu::{PipelineLayout, RenderPipeline};

mod solid;
mod wireframe;
mod world;
pub enum Pipeline {
    Wireframe,
    Solid,
    World,
}

impl Pipeline {
    pub fn get(
        &self,
        context: &WgpuContext,
        layout: PipelineLayout,
    ) -> RenderPipeline {
        match self {
            Pipeline::Wireframe => wireframe::get(context, layout),
            Pipeline::Solid => solid::get(context, layout),
            Pipeline::World => world::get(context, layout),
        }
    }
}
