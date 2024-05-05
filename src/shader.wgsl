struct BasicMesh {
    p1: vec2f,
    p2: vec2f,
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    @location(0) vert_pos_2d: vec2<f32>    
) -> @builtin(position) vec4<f32> {
    return vec4<f32>(vert_pos_2d.x, vert_pos_2d.y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}