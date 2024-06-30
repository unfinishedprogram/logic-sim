pub mod draw;
mod render_queue;

pub use render_queue::RenderQueue;

use crate::game::input::InputState;

use super::{
    camera::Camera, msdf::sprite_renderer::SpriteRendererReference, vector::VectorRendererReference,
};

// Immediate mode context for a frame
pub struct Frame {
    camera: Camera,
    ui_camera: Camera,
    input_state: InputState,
    pub assets: FrameAssets,
    render_queue: RenderQueue,
    pub ui_render_queue: RenderQueue,
}

pub struct FrameAssets {
    pub sprites: SpriteRendererReference,
    pub vectors: VectorRendererReference,
}

impl Frame {
    pub fn new(
        camera: Camera,
        ui_camera: Camera,
        input: &InputState,
        sprites: SpriteRendererReference,
        vectors: VectorRendererReference,
    ) -> Self {
        Self {
            input_state: input.clone(),
            camera,
            ui_camera,
            assets: FrameAssets { sprites, vectors },
            render_queue: RenderQueue::new(),
            ui_render_queue: RenderQueue::new(),
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn ui_camera(&self) -> &Camera {
        &self.ui_camera
    }

    pub fn input(&self) -> &InputState {
        &self.input_state
    }

    pub fn render_queue(&self) -> &RenderQueue {
        &self.render_queue
    }
}
