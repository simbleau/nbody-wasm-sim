use super::WgpuContext;
use wgpu::RenderPipeline;

mod solid;
mod wireframe;

pub enum Pipeline {
    Wireframe,
    Solid,
}

pub fn get(pipeline: Pipeline, context: &WgpuContext) -> RenderPipeline {
    match pipeline {
        Pipeline::Wireframe => wireframe::get(context),
        Pipeline::Solid => solid::get(context),
    }
}
