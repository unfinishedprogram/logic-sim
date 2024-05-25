struct Camera {
    center: vec2f,
    size: vec2f,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct VertexOutput {
    @builtin(position) clip_pos: vec4f,
    @location(0) tex_coords: vec2<f32>,
}

struct VertexInput {
    @builtin(vertex_index) in_vertex_index: u32,
    @location(0) vert_pos_2d: vec2<f32>,
    @location(1) uv_pos_2d: vec2<f32>,
}

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_pos = vec4f((in.vert_pos_2d * vec2f(1.0, -1.0) - camera.center) / camera.size, 0.0, 1.0);
    out.tex_coords = in.uv_pos_2d;
    return out;
}

fn sampleMsdf(texcoord: vec2f) -> f32 {
    let c = textureSample(t_diffuse, s_diffuse, texcoord);
    return max(min(c.r, c.g), min(max(c.r, c.g), c.b));
}

fn screenPxRange(pxRange: f32, texCoord: vec2f) -> f32 {
    let unitRange = vec2f(pxRange) / vec2f(textureDimensions(t_diffuse, 0));
    let screenTexSize = vec2(1.0)/fwidth(texCoord);
    return max(0.5*dot(unitRange, screenTexSize), 1.0);
}

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4f {
    let sd = sampleMsdf(in.tex_coords);

    let screenPxDistance = screenPxRange(4.0, in.tex_coords)*(sd - 0.5);
    let opacity = clamp(screenPxDistance + 0.5, 0.0, 1.0);

    return vec4f(opacity);
}

