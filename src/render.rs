mod bindable;
pub mod camera;
pub mod frame;
pub mod geometry;
mod img_texture;
pub mod line;
pub mod msdf;
pub mod vector;
pub mod vertex;
use frame::Frame;
use msdf::text::MsdfFontReference;
use vector::VectorRenderer;
use wgpu::{Adapter, Color, Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

use self::{
    line::LineRenderer,
    msdf::{sprite::sprite_sheet::SpriteSheet, sprite_renderer::SpriteRenderer, text::MsdfFont},
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
    adapter: Adapter,
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
            include_str!("../assets/custom-msdf.json"),
            include_bytes!("../assets/custom.png"),
        );

        let dot_sprite_sheet = SpriteSheet::create(
            &base.device,
            &base.queue,
            &serde_json::from_str(include_str!("../assets/dot/manifest.json")).unwrap(),
            include_bytes!("../assets/dot/spritesheet-msdf.png"),
        );

        let msdf_font_ref = msdf_font.reference(0);

        let sprite_renderer =
            SpriteRenderer::create(&base, vec![msdf_font.sprite_sheet, dot_sprite_sheet]);

        let line_renderer = line::LineRenderer::create(&base);
        let mut vector_renderer = vector::VectorRenderer::create(&base);

        let gate_assets = ["and", "buf", "nand", "nor", "not", "or", "xor", "xnor"];

        for name in gate_assets {
            vector_renderer
                .load_svg(&format!("assets/objects/gates/{}.svg", name), Some(name))
                .unwrap();
        }

        Self {
            base,
            sprite_renderer,
            msdf_font_ref,
            line_renderer,
            vector_renderer,
        }
    }

    pub fn render(&mut self, frame: Frame) {
        let camera = frame.camera();
        let lines = frame.lines();
        let sprites = frame.sprites();
        let vector_instances = frame.vector_instances();

        let surface = self
            .base
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let frame_view = surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let view = self.create_multisampled_frame_buffer(4);

        let mut encoder = self
            .base
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: Some(&frame_view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.line_renderer.update_camera(&self.base.queue, camera);
            self.sprite_renderer.update_camera(&self.base.queue, camera);
            self.vector_renderer.update_camera(&self.base.queue, camera);

            self.line_renderer
                .upload_geometry(&self.base.queue, &lines.indices, &lines.vertices);

            self.sprite_renderer
                .upload_sprites(&self.base.queue, sprites);

            self.vector_renderer
                .upload_instances(&self.base.queue, vector_instances);

            self.line_renderer.render(&mut rpass);
            self.sprite_renderer.render(&mut rpass);
            self.vector_renderer.render(&mut rpass);
        }

        self.base.queue.submit(Some(encoder.finish()));

        surface.present();
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
            adapter,
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
}
