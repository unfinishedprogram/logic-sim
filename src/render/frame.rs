pub mod draw;
mod render_queue;

use glam::Vec2;
pub use render_queue::RenderQueue;

use crate::game::input::InputState;

use super::{
    camera::Camera,
    msdf::{sprite_renderer::SpriteRendererReference, text::MsdfFontReference},
    vector::VectorRendererReference,
};

// Immediate mode context for a frame
pub struct Frame {
    camera: Camera,
    ui_camera: Camera,
    input_state: InputState,
    resolution: Vec2,
    pub assets: FrameAssets,
    pub render_queue: RenderQueue,
    pub ui_render_queue: RenderQueue,
}

pub struct FrameAssets {
    pub sprites: SpriteRendererReference,
    pub vectors: VectorRendererReference,
    pub font: MsdfFontReference,
}

impl Frame {
    pub fn new(
        camera: Camera,
        ui_camera: Camera,
        input: &InputState,
        assets: FrameAssets,
        resolution: Vec2,
    ) -> Self {
        Self {
            input_state: input.clone(),
            camera,
            ui_camera,
            assets,
            render_queue: RenderQueue::default(),
            ui_render_queue: RenderQueue::default(),
            resolution,
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn world_pixel_size(&self) -> Vec2 {
        self.camera.size / self.resolution
    }

    pub fn ui_camera(&self) -> &Camera {
        &self.ui_camera
    }

    pub fn input(&self) -> &InputState {
        &self.input_state
    }
}
