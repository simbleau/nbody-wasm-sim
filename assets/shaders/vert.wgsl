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
    @location(1) color: vec3<f32>,
};

struct InstanceInput {
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
    @location(6) radius: f32,
};

struct WorldUniform {
    radius: f32,
    boundary_segments: u32,
    rave: u32,
    padding: u32,
};

@group(2) @binding(0)
var<uniform> world: WorldUniform;

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

    var brightest: vec3<f32> = vec3<f32>(0.97, 0.97, 1.0);
    var blue: vec3<f32> = vec3<f32>(0.33, 0.4, 1.0);
    var yellow: vec3<f32> = vec3<f32>(0.97, 0.98, 0.8);
    var orange: vec3<f32> = vec3<f32>(0.96, 0.6, 0.25);
    var red: vec3<f32> = vec3<f32>(0.99, 0.25, 0.25);

    var star_color: vec3<f32> = vec3<f32>(0.0);
    if (instance.radius >= 0.75) {
        star_color += mix(orange, red, (instance.radius - 0.75) * 4.0);
    } else if (instance.radius >= 0.5) {
        star_color += mix(yellow, orange, (instance.radius - 0.5) * 4.0);
    } else if (instance.radius >= 0.25) {
        star_color += mix(blue, yellow, (instance.radius - 0.25) * 4.0);
    } else {
        star_color += mix(brightest, blue, instance.radius * 4.0);
    }

    var tint: vec3<f32> = vec3<f32>(1.0);
    var twinkle: vec3<f32> = vec3<f32>(1.0);
    if (world.rave > 0u) {
        tint = vec3<f32>(abs(world_vert.xyz) % 2.0);
        twinkle = vec3<f32>(instance.model_matrix_0.x);
    }


    // Calculate color
    out.color = star_color * tint * twinkle;
    return out;
}