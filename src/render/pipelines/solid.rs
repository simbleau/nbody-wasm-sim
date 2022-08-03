use wgpu::PipelineLayout;

use crate::render::gpu_types::{GpuPrimitive, GpuQuad, GpuTransform};

pub(crate) fn get(
    context: &crate::render::WgpuContext,
    layout: PipelineLayout,
) -> wgpu::RenderPipeline {
    let vert_shader = context.get_shader("vert");
    let frag_shader = context.get_shader("frag");

    let pipeline = context.device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("Solid Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "vs_main",
                buffers: &[GpuQuad::desc(), GpuTransform::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: context.config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires
                // Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        },
    );

    pipeline
}
