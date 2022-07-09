struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
