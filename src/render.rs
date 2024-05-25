mod bindable;
pub mod camera;
pub mod geometry;
mod img_texture;
pub mod msdf;
pub mod vertex;
use wgpu::{Adapter, Color, Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

use self::{
    camera::CameraBinding,
    msdf::{
        sprite::sprite_sheet::{SpriteInstance, SpriteSheet},
        sprite_renderer::SpriteRenderer,
        text::MsdfFont,
    },
};

pub struct RenderState<'window> {
    pub base: BaseRenderState<'window>,
    pub sprite_renderer: SpriteRenderer,
    pub msdf_font: MsdfFont,
}

pub struct BaseRenderState<'window> {
    surface: Surface<'window>,
    pub surface_config: SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    pub queue: Queue,
    swapchain_format: wgpu::TextureFormat,
}

pub struct BindingState {
    pub camera: CameraBinding,
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

        let other_font = MsdfFont::create(
            &base.device,
            &base.queue,
            include_str!("../assets/custom-msdf.json"),
            include_bytes!("../assets/custom.png"),
        );

        let gates_sprite_sheet = SpriteSheet::create(
            &base.device,
            &base.queue,
            &serde_json::from_str(include_str!("../assets/gates/manifest.json")).unwrap(),
            include_bytes!("../assets/gates/spritesheet-msdf.png"),
        );

        let sprite_renderer =
            SpriteRenderer::create(&base, vec![other_font.sprite_sheet, gates_sprite_sheet]);

        Self {
            base,
            msdf_font,
            sprite_renderer,
        }
    }

    pub fn render(&mut self, sprites: &Vec<SpriteInstance>) {
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
            let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

            self.sprite_renderer
                .upload_sprites(&self.base.queue, &sprites);

            // rpass.set_bind_group(0, self.binding_state.camera.bind_group(), &[]);
            self.sprite_renderer.render(rpass);
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

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        Self {
            surface,
            surface_config,
            adapter,
            device,
            queue,
            swapchain_format,
        }
    }

    fn resize(&mut self, _window: &Window, new_size: PhysicalSize<u32>) {
        // Reconfigure the surface with the new size
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);

        self.surface.configure(&self.device, &self.surface_config);
    }
}
