mod bindable;
pub mod camera;
pub mod frame;
pub mod geometry;
pub mod helpers;
mod img_texture;
pub mod line;
pub mod msdf;
pub mod vector;
pub mod vertex;
use std::any::type_name;

use crate::assets;
use camera::Camera;
use frame::{Frame, RenderQueue};
use msdf::text::MsdfFontReference;
use vector::VectorRenderer;
use wgpu::{
    Device, Queue, ShaderModule, ShaderModuleDescriptor, Surface, SurfaceConfiguration, TextureView,
};
use winit::{dpi::PhysicalSize, window::Window};

use self::{
    line::LineRenderer,
    msdf::{sprite_renderer::SpriteRenderer, text::MsdfFont},
};

pub struct RenderState<'window> {
    pub base: BaseRenderState<'window>,
    pub msdf_font_ref: MsdfFontReference,
    // Render pipelines
    pub line_renderer: LineRenderer,
    pub sprite_renderer: SpriteRenderer,
    pub vector_renderer: VectorRenderer,
}

pub struct BaseRenderState<'window> {
    surface: Surface<'window>,
    pub surface_config: SurfaceConfiguration,
    device: Device,
    pub queue: Queue,
    swapchain_format: wgpu::TextureFormat,
    msaa_config: wgpu::MultisampleState,
}

impl<'window> RenderState<'window> {
    pub async fn create(window: &'window Window) -> Self {
        let base = BaseRenderState::create(window).await;

        let msdf_font = MsdfFont::create(
            &base.device,
            &base.queue,
            assets::fonts::msdf::custom::MANIFEST,
            assets::fonts::msdf::custom::IMAGE,
        );

        let msdf_font_ref = msdf_font.reference();

        let sprite_renderer = SpriteRenderer::create(&base, vec![msdf_font.sprite_sheet]);
        let line_renderer = line::LineRenderer::create(&base);
        let vector_renderer = vector::VectorRenderer::create(&base);

        Self {
            base,
            sprite_renderer,
            msdf_font_ref,
            line_renderer,
            vector_renderer,
        }
    }

    pub fn render(&mut self, frame: Frame) {
        let surface = self
            .base
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let frame_view = surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let msaa_view = self.create_multisampled_frame_buffer(4);

        let attachments = Self::color_attachments(&msaa_view, &frame_view);
        let render_pass_desc = Self::frame_render_pass_descriptor(&attachments);

        self.render_world(&frame, &render_pass_desc);
        self.render_ui(&frame, &render_pass_desc);

        surface.present();
    }

    fn render_world(&mut self, frame: &Frame, render_pass_desc: &wgpu::RenderPassDescriptor) {
        self.upload_resources(frame.camera(), frame.render_queue());

        let mut encoder = self
            .base
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(render_pass_desc);

            self.line_renderer.render(&mut rpass);
            self.sprite_renderer.render(&mut rpass);
            self.vector_renderer.render(&mut rpass);
        }

        self.base.queue.submit(Some(encoder.finish()));
    }

    fn render_ui(&mut self, frame: &Frame, render_pass_desc: &wgpu::RenderPassDescriptor) {
        self.upload_resources(&frame.ui_camera(), frame.ui_render_queue());

        let mut encoder = self
            .base
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(render_pass_desc);

            self.line_renderer.render(&mut rpass);
            self.sprite_renderer.render(&mut rpass);
            self.vector_renderer.render(&mut rpass);
        }

        self.base.queue.submit(Some(encoder.finish()));
    }

    pub fn resize(&mut self, window: &Window, new_size: PhysicalSize<u32>) {
        self.base.resize(window, new_size);
    }

    fn create_multisampled_frame_buffer(&self, sample_count: u32) -> wgpu::TextureView {
        let extend = wgpu::Extent3d {
            width: self.base.surface_config.width,
            height: self.base.surface_config.height,
            depth_or_array_layers: 1,
        };

        let descriptor = &wgpu::TextureDescriptor {
            label: Some("MSAA Frame Buffer"),
            size: extend,
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: self.base.swapchain_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[self.base.swapchain_format],
        };

        self.base
            .device
            .create_texture(descriptor)
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn upload_resources(&mut self, camera: &Camera, render_queue: &RenderQueue) {
        self.line_renderer.update_camera(&self.base.queue, camera);
        self.sprite_renderer.update_camera(&self.base.queue, camera);
        self.vector_renderer.update_camera(&self.base.queue, camera);

        let lines = render_queue.lines();
        let sprites = render_queue.sprites();
        let vector_instances = render_queue.vector_instances();
        let lazy_vector_instances = render_queue.lazy_vector_instances();

        self.line_renderer
            .upload_geometry(&self.base.queue, &lines.indices, &lines.vertices);

        self.sprite_renderer
            .upload_sprites(&self.base.queue, sprites);

        self.vector_renderer.upload_instances(
            &self.base.queue,
            vector_instances,
            lazy_vector_instances,
        );
    }

    fn color_attachments<'a>(
        msaa_view: &'a TextureView,
        frame_view: &'a TextureView,
    ) -> [Option<wgpu::RenderPassColorAttachment<'a>>; 1] {
        [Some(wgpu::RenderPassColorAttachment {
            view: msaa_view,
            resolve_target: Some(frame_view),
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        })]
    }

    fn frame_render_pass_descriptor<'a>(
        color_attachments: &'a [Option<wgpu::RenderPassColorAttachment>],
    ) -> wgpu::RenderPassDescriptor<'a, 'a> {
        wgpu::RenderPassDescriptor {
            label: None,
            color_attachments,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        }
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

        // surface_config.present_mode = wgpu::PresentMode::AutoVsync;
        surface_config.alpha_mode = wgpu::CompositeAlphaMode::PreMultiplied;

        surface.configure(&device, &surface_config);

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let msaa_config = wgpu::MultisampleState {
            count: 4,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        Self {
            surface,
            surface_config,
            device,
            queue,
            swapchain_format,
            msaa_config,
        }
    }

    fn resize(&mut self, _window: &Window, new_size: PhysicalSize<u32>) {
        // Reconfigure the surface with the new size
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);

        self.surface.configure(&self.device, &self.surface_config);
    }

    // Helpers
    pub fn create_shader_module(&self, desc: ShaderModuleDescriptor<'_>) -> ShaderModule {
        self.device.create_shader_module(desc)
    }

    pub fn create_vertex_buffer<T>(&self, size: u64) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{}: Vertex Buffer", type_name::<T>())),
            size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn create_instance_buffer<T>(&self, size: u64) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{}: Instance Buffer", type_name::<T>())),
            size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn create_index_buffer<T>(&self, size: u64) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{}: Index Buffer", type_name::<T>())),
            size,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}
