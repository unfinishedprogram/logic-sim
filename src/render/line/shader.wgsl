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
    @location(1) color: vec4<f32>,
}

struct VertexInput {
    @builtin(vertex_index) in_vertex_index: u32,
    @location(0) vert_pos_2d: vec2<f32>,
    @location(1) uv_pos_2d: vec2<f32>,
    @location(2) color: vec4<f32>,
}

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_pos = vec4f((in.vert_pos_2d - camera.center) / camera.size, 0.0, 1.0) * vec4f(1.0, -1.0, 1.0, 1.0);
    out.tex_coords = in.uv_pos_2d;
    out.color = in.color;
    return out;
}



@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4f {
    return in.color;
}

