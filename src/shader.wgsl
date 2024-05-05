struct Camera {
    center: vec2f,
    size: vec2f,
}

@group(0) @binding(0) // 1.
var<uniform> camera: Camera;

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    @location(0) vert_pos_2d: vec2f    
) -> @builtin(position) vec4f {

    let real_pos = (vert_pos_2d - camera.center) / camera.size;

    return vec4f(real_pos, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4f {
    return vec4f(1.0, 0.0, 0.0, 1.0);
}