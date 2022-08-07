struct Input {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct Output {
    @location(0) color: vec4<f32>,
};

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

// Fragment shader
@fragment
fn fs_main(in: Input) -> Output {
    var out: Output;
    out.color = textureSample(texture, texture_sampler, vec2<f32>(1.0 - in.uv.x, 1.0 - in.uv.y));
    if ((pow(in.uv.x - 0.5, 2.0) + pow(in.uv.y - 0.5, 2.0)) > pow(0.5, 2.0)) {
        out.color = vec4<f32>(0.0);
    }

    out.color *= vec4<f32>(in.color, 1.0);
    return out;
}
