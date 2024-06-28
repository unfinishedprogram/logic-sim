use lyon::tessellation::VertexBuffers;
use wgpu::{ColorTargetState, PipelineLayout, RenderPipelineDescriptor, ShaderModule};

pub fn generic_pipeline_descriptor<'a>(
    layout: &'a PipelineLayout,
    shader: &'a ShaderModule,
    targets: &'a [Option<ColorTargetState>],
    buffers: &'a [wgpu::VertexBufferLayout<'a>],
    multisample: wgpu::MultisampleState,
) -> RenderPipelineDescriptor<'a> {
    wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            targets,
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample,
        multiview: None,
    }
}

pub fn join_buffers<OutVert>(
    buffers: Vec<VertexBuffers<OutVert, u32>>,
) -> VertexBuffers<OutVert, u32> {
    let (num_vertices, num_indices) = buffers.iter().fold((0, 0), |(v, i), buffer| {
        (v + buffer.vertices.len(), i + buffer.indices.len())
    });

    buffers.into_iter().fold(
        VertexBuffers::with_capacity(num_vertices, num_indices),
        |mut acc, buffer| {
            extend_vertex_buffer(&mut acc, buffer);
            acc
        },
    )
}

pub fn extend_vertex_buffer<OutVert>(
    into: &mut VertexBuffers<OutVert, u32>,
    from: VertexBuffers<OutVert, u32>,
) {
    let index_offset = into.vertices.len() as u32;

    into.vertices.extend(from.vertices);
    into.indices
        .extend(from.indices.into_iter().map(|index| index + index_offset));
}
