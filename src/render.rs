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

use self::{
    line::LineRenderer,
    msdf::{sprite_renderer::SpriteRenderer, text::MsdfFont},
};
use camera::Camera;
use frame::{Frame, RenderQueue};
use msdf::text::MsdfFontReference;
use vector::VectorRenderer;
use wgpu::{
    Device, Queue, ShaderModule, ShaderModuleDescriptor, Surface, SurfaceConfiguration, TextureView,
};
use winit::{dpi::PhysicalSize, window::Window};

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

    pub fn render(&mut self, mut frame: Frame) {
        let tolerance = 0.001 * f32::max(frame.camera().size.x, 1.0);
        frame.render_queue.tesselate_geometry(tolerance);

        let tolerance_ui = 0.001 * f32::max(frame.ui_camera().size.x, 1.0);
        frame.ui_render_queue.tesselate_geometry(tolerance_ui);

        let surface = self
            .base
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let frame_view = surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let msaa_view = self.base.create_multisampled_frame_buffer(4);

        {
            let attachments = Self::color_attachments(&msaa_view, &frame_view, true);
            let render_pass_desc = Self::frame_render_pass_descriptor(&attachments);
            self.upload_resources(frame.camera(), &frame.render_queue);
            self.render_internal(&render_pass_desc);
        }

        {
            let attachments = Self::color_attachments(&msaa_view, &frame_view, false);
            let render_pass_desc = Self::frame_render_pass_descriptor(&attachments);
            self.upload_resources(frame.ui_camera(), &frame.ui_render_queue);
            self.render_internal(&render_pass_desc);
        }

        surface.present();
    }

    // Resources must be uploaded before render_internal is called
    fn render_internal(&mut self, render_pass_desc: &wgpu::RenderPassDescriptor) {
        let mut encoder = self
            .base
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(render_pass_desc);

            self.line_renderer.render(&mut rpass);
            self.vector_renderer.render(&mut rpass);
            self.sprite_renderer.render(&mut rpass);
        }

        self.base.queue.submit(Some(encoder.finish()));
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.base.resize(new_size);
    }

    fn upload_resources(&mut self, camera: &Camera, render_queue: &RenderQueue) {
        self.line_renderer.update_camera(&self.base.queue, camera);
        self.sprite_renderer.update_camera(&self.base.queue, camera);
        self.vector_renderer.update_camera(&self.base.queue, camera);

        let lines = render_queue.lines();
        let sprites = render_queue.sprites();
        let vector_instances = render_queue.vector_instances();
        let lazy_vector_instances = render_queue.lazy_vector_instances();

        self.line_renderer.upload_geometry(&self.base.queue, lines);

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
        clear: bool,
    ) -> [Option<wgpu::RenderPassColorAttachment<'a>>; 1] {
        let load = if clear {
            wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0,
            })
        } else {
            wgpu::LoadOp::Load
        };

        [Some(wgpu::RenderPassColorAttachment {
            view: msaa_view,
            resolve_target: Some(frame_view),
            ops: wgpu::Operations {
                load,
                store: wgpu::StoreOp::Store,
            },
        })]
    }

    fn frame_render_pass_descriptor<'a>(
        color_attachments: &'a [Option<wgpu::RenderPassColorAttachment>],
    ) -> wgpu::RenderPassDescriptor<'a> {
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
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let mut surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface_config.present_mode = wgpu::PresentMode::AutoVsync;
        surface_config.alpha_mode = wgpu::CompositeAlphaMode::Opaque;

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

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        // Reconfigure the surface with the new size
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);

        self.surface.configure(&self.device, &self.surface_config);
    }

    fn create_multisampled_frame_buffer(&self, sample_count: u32) -> wgpu::TextureView {
        let extend = wgpu::Extent3d {
            width: self.surface_config.width,
            height: self.surface_config.height,
            depth_or_array_layers: 1,
        };

        let descriptor = &wgpu::TextureDescriptor {
            label: Some("MSAA Frame Buffer"),
            size: extend,
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: self.swapchain_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[self.swapchain_format],
        };

        self.device
            .create_texture(descriptor)
            .create_view(&wgpu::TextureViewDescriptor::default())
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
