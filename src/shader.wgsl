struct Camera {
    center: vec2f,
    size: vec2f,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(2) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(2) @binding(1)
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

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4f {
    // pxRange (AKA distanceRange) comes from the msdfgen tool. Don McCurdy's tool
    // uses the default which is 4.
    let pxRange = 4.0;
    let sz = vec2f(textureDimensions(t_diffuse, 0));
    let dx = sz.x*length(vec2f(dpdxFine(in.tex_coords.x), dpdyFine(in.tex_coords.x)));
    let dy = sz.y*length(vec2f(dpdxFine(in.tex_coords.y), dpdyFine(in.tex_coords.y)));
    let toPixels = pxRange * inverseSqrt(dx * dx + dy * dy);
    let sigDist = sampleMsdf(in.tex_coords) - 0.5;
    let pxDist = sigDist * toPixels;

    let edgeWidth = 0.5;

    let alpha = smoothstep(-edgeWidth, edgeWidth, pxDist);

    return vec4f(alpha);
}

