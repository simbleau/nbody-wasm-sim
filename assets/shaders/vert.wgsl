struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct Output {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

// Vertex shader
@vertex
fn vs_main(
    input: VertexInput,
) -> Output {
    var out: Output;
    out.color = vec4<f32>(input.color, 1.0);
    out.clip_position = vec4<f32>(input.position.x, input.position.y, input.position.z, 1.0);
    return out;
}