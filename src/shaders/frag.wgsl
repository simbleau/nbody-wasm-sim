struct VertexInput {
    @builtin(position) clip_position: vec4<f32>,
};

struct FragmentOutput {
    @builtin(position) color: vec4<f32>,
};

// Fragment shader
@fragment
fn fs_main(in: VertexInput) -> @location(0) FragmentOutput {
    var output: FragmentOutput;
    output.color = vec4<f32>(0.3, 0.2, 0.1, 1.0);
    output
}
