struct Input {
    @builtin(position) clip_position: vec4<f32>,
};

struct Output {
    @location(0) color: vec4<f32>,
};

// Fragment shader
@fragment
fn fs_main(in: Input) -> Output {
    var out: Output;
    out.color = vec4<f32>(1.0);
    return out;
}