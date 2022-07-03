use wgpu::{RenderPassColorAttachment, TextureView};

use crate::state::State;

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
