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
