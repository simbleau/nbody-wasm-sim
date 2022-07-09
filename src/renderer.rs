use wgpu::{
    CommandEncoder, RenderPass, RenderPassColorAttachment, RenderPipeline,
    TextureView,
};

use crate::{
    frame_description::GpuTriangle, state::State, wgpu_context::WgpuContext,
};

pub fn get_pipeline(context: &WgpuContext) -> RenderPipeline {
    let vert_shader = context.shaders.get("vert").unwrap();
    let frag_shader = context.shaders.get("frag").unwrap();

    let pipeline_layout = context.device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        },
    );

    let pipeline = context.device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "vs_main",
                buffers: &[GpuTriangle::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: context.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
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

pub fn get_render_pass<'pass>(
    encoder: &'pass mut CommandEncoder,
    state: &'pass State,
    view: &'pass TextureView,
) -> RenderPass<'pass> {
    let attachments = get_attachments(state, &view);
    let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[attachments],
        depth_stencil_attachment: None,
    });

    pass
}

pub fn get_attachments<'a>(
    state: &'a State,
    view: &'a TextureView,
) -> Option<RenderPassColorAttachment<'a>> {
    let color = wgpu::Color {
        r: state.mouse_pos.x / state.window_size.y as f64,
        g: state.mouse_pos.x / state.window_size.y as f64,
        b: state.mouse_pos.y / state.window_size.y as f64,
        a: 1.0,
    };

    Some(wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(color),
            store: true,
        },
    })
}
