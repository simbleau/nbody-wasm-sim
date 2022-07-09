use wgpu::{
    CommandEncoder, RenderPass, RenderPassColorAttachment, TextureView,
};

use crate::state::State;

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
