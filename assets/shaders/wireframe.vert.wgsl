struct Input {
    @location(0) position: vec3<f32>,
}

struct Output {
    @builtin(position) clip_position: vec4<f32>,
};

// Vertex shader
@vertex
fn vs_main(
    model: Input,
) -> Output {
    var out: Output;
    out.clip_position = vec4<f32>(model.position.xyz, 1.0);
    return out;
}