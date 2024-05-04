use std::borrow::Cow;

use wgpu::{Adapter, Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

pub struct RenderState<'window> {
    base: BaseRenderState<'window>,
    render_pipeline: RenderPipeline,
}

pub struct BaseRenderState<'window> {
    instance: Instance,
    surface: Surface<'window>,
    surface_config: SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl<'window> RenderState<'window> {
    pub async fn create(window: &'window Window) -> Self {
        let base = BaseRenderState::create(window).await;

        // Load the shaders from disk
        let shader = base
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
            });

        let pipeline_layout = base
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let swapchain_capabilities = base.surface.get_capabilities(&base.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = base
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Self {
            base,
            render_pipeline,
        }
    }

    pub fn render(&mut self) {
        let frame = self
            .base
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .base
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&self.render_pipeline);
            rpass.draw(0..3, 0..1);
        }

        self.base.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, window: &Window, new_size: PhysicalSize<u32>) {
        self.base.resize(window, new_size);
    }
}

impl<'window> BaseRenderState<'window> {
    pub async fn create(window: &'window Window) -> Self {
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &surface_config);

        Self {
            instance,
            surface,
            surface_config,
            adapter,
            device,
            queue,
        }
    }

    fn resize(&mut self, window: &Window, new_size: PhysicalSize<u32>) {
        // Reconfigure the surface with the new size
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);

        self.surface.configure(&self.device, &self.surface_config);
        // On macos the window needs to be redrawn manually after resizing
        window.request_redraw();
    }
}

// async fn run(event_loop: EventLoop<()>, window: Window) {

//     // Load the shaders from disk
//     let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//         label: None,
//         source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
//     });

//     let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//         label: None,
//         bind_group_layouts: &[],
//         push_constant_ranges: &[],
//     });

//     let swapchain_capabilities = surface.get_capabilities(&adapter);
//     let swapchain_format = swapchain_capabilities.formats[0];

//     let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//         label: None,
//         layout: Some(&pipeline_layout),
//         vertex: wgpu::VertexState {
//             module: &shader,
//             entry_point: "vs_main",
//             buffers: &[],
//         },
//         fragment: Some(wgpu::FragmentState {
//             module: &shader,
//             entry_point: "fs_main",
//             targets: &[Some(swapchain_format.into())],
//         }),
//         primitive: wgpu::PrimitiveState::default(),
//         depth_stencil: None,
//         multisample: wgpu::MultisampleState::default(),
//         multiview: None,
//     });

//     let mut config = surface
//         .get_default_config(&adapter, size.width, size.height)
//         .unwrap();
//     surface.configure(&device, &config);
// }
