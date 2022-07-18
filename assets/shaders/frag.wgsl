struct Input {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Output {
    @location(0) color: vec4<f32>,
};

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0)@binding(1)
var texture_sampler: sampler;

// Fragment shader
@fragment
fn fs_main(in: Input) -> Output {
    var out: Output;
    out.color = textureSample(texture, texture_sampler, vec2<f32>(in.uv.x, 1.0 - in.uv.y));
    return out;
}
