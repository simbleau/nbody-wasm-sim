struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct WorldUniform {
    radius: f32,
    boundary_segments: u32,
    rave: u32,
    padding: u32,
};

@group(1) @binding(0)
var<uniform> world: WorldUniform;

struct Output {
    @builtin(position) clip_position: vec4<f32>,
};

// Vertex shader
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> Output {
    var pi: f32 = 3.1415926538;
    var i: u32 = in_vertex_index;

    var delta_radians = 2.0 * pi / f32(world.boundary_segments);
    var x: f32 = cos(delta_radians * f32(i));
    var y: f32 = sin(delta_radians * f32(i));

    var out: Output;

    var world_vert = vec4<f32>(x * world.radius, y * world.radius, 0.0, 1.0);

    // World coords -> Device Coordinates
    out.clip_position = camera.view_proj * world_vert;

    return out;
}