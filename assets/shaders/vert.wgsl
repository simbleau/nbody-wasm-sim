struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct Input {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct Output {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct InstanceInput {
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
};

// Vertex shader
@vertex
fn vs_main(
    model: Input,
    instance: InstanceInput,
) -> Output {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: Output;
    out.uv = model.uv;

    // Local coords -> World coords
    var world_vert: vec4<f32> = (model_matrix * vec4<f32>(model.position, 1.0));

    // World coords -> Device Coordinates
    out.clip_position = camera.view_proj * world_vert;
    return out;
}