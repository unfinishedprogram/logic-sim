mod bindable;
mod camera;
pub mod geometry;
mod img_texture;
pub mod msdf;
mod quad;
pub mod vertex;
use glam::Vec2;
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    Adapter, Buffer, BufferUsages, Color, Device, Queue, RenderPipeline, Surface,
    SurfaceConfiguration,
};
use winit::{dpi::PhysicalSize, window::Window};

use self::{
    bindable::{BindList, BindTarget},
    camera::Camera,
    msdf::text::{MsdfFont, TextObject},
    vertex::VertexUV,
};

pub struct RenderState<'window> {
    base: BaseRenderState<'window>,
    render_pipeline: RenderPipeline,
    vertex_buf: Buffer,
    pub text_object: TextObject,
    pub binding_state: BindingState,
}

pub struct BaseRenderState<'window> {
    surface: Surface<'window>,
    surface_config: SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

pub struct BindingState {
    pub camera: Camera,
    pub msdf_font: MsdfFont,
}

impl<'a> From<&'a BindingState> for BindList<'a> {
    fn from(binding_state: &'a BindingState) -> Self {
        let mut bind_list = BindList::new();
        bind_list.push(&binding_state.camera);
        bind_list.push(&binding_state.msdf_font);
        bind_list.push(binding_state.msdf_font.texture());
        bind_list
    }
}

impl<'window> RenderState<'window> {
    pub async fn create(window: &'window Window) -> Self {
        let text_object = TextObject {
            content: "Hello, World!".to_string(),
            position: Vec2::new(0.0, -1.0),
            scale: 1.0,
        };

        let base = BaseRenderState::create(window).await;

        let mut camera = Camera::create(&base.device);

        camera.set_aspect(
            base.surface_config.width as f32 / base.surface_config.height as f32,
            10.0,
        );

        let msdf_font = MsdfFont::create(
            &base.device,
            &base.queue,
            include_str!("../assets/custom-msdf.json"),
            include_bytes!("../assets/custom.png"),
        );

        let binding_state = BindingState { camera, msdf_font };
        let bind_list = BindList::from(&binding_state);

        let shader = base
            .device
            .create_shader_module(include_wgsl!("shader.wgsl"));

        let bind_group_layouts = bind_list.bind_group_layouts();

        let pipeline_layout = base
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &bind_group_layouts,
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
                    buffers: &[VertexUV::buffer_layout_descriptor()],
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

        let text_quads = text_object.as_textured_quads(&binding_state.msdf_font);

        let vertex_buf = base.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&text_quads),
            usage: BufferUsages::VERTEX,
        });

        Self {
            base,
            render_pipeline,
            vertex_buf,
            text_object,
            binding_state,
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

        let bind_list = BindList::from(&self.binding_state);
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));

            rpass.set_bind_groups(&bind_list);

            rpass.set_pipeline(&self.render_pipeline);

            rpass.draw(0..self.text_object.content.len() as u32 * 6, 0..1);
        }

        self.base.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, window: &Window, new_size: PhysicalSize<u32>) {
        let old_width = self.base.surface_config.width as f32;
        let old_height = self.base.surface_config.height as f32;

        let new_width = new_size.width as f32;
        let new_height = new_size.height as f32;

        let scale = (new_width / old_width, new_height / old_height);
        self.binding_state.camera.scale(scale.into());
        self.base.resize(window, new_size);
    }

    pub fn update_camera(&self) {
        self.binding_state.camera.update(&self.base.queue);
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
                    required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let mut surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface_config.present_mode = wgpu::PresentMode::AutoVsync;
        surface_config.alpha_mode = wgpu::CompositeAlphaMode::PreMultiplied;

        surface.configure(&device, &surface_config);

        Self {
            surface,
            surface_config,
            adapter,
            device,
            queue,
        }
    }

    fn resize(&mut self, _window: &Window, new_size: PhysicalSize<u32>) {
        // Reconfigure the surface with the new size
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);

        self.surface.configure(&self.device, &self.surface_config);
    }
}
