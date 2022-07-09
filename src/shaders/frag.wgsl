struct VertexInput {
    @builtin(position) clip_position: vec4<f32>,
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
};

// Fragment shader
@fragment
fn fs_main(in: VertexInput) -> FragmentOutput {
    var out: FragmentOutput;
    out.color = vec4<f32>(0.3, 0.2, 0.1, 1.0);
    return out;
}
