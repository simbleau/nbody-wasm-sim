use super::WgpuContext;
use wgpu::{PipelineLayout, RenderPipeline};

mod solid;
mod wireframe;

pub enum Pipeline {
    Wireframe,
    Solid,
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
        }
    }
}
