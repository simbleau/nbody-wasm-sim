struct Input {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
};

// Fragment shader
@fragment
fn fs_main(in: Input) -> FragmentOutput {
    var out: FragmentOutput;
    out.color = in.color;
    return out;
}
