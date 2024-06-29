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
    input_state: InputState,
    pub assets: FrameAssets,
    render_queue: RenderQueue,
    ui_render_queue: RenderQueue,
}

pub struct FrameAssets {
    pub sprites: SpriteRendererReference,
    pub vectors: VectorRendererReference,
}

impl Frame {
    pub fn new(
        camera: &Camera,
        input: &InputState,
        sprites: SpriteRendererReference,
        vectors: VectorRendererReference,
    ) -> Self {
        Self {
            input_state: input.clone(),
            camera: *camera,
            assets: FrameAssets { sprites, vectors },
            render_queue: RenderQueue::new(),
            ui_render_queue: RenderQueue::new(),
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn ui_camera(&self) -> Camera {
        let mut cam = Camera::new();
        cam.translate(-cam.top_left());
        cam
    }

    pub fn input(&self) -> &InputState {
        &self.input_state
    }

    pub fn render_queue(&self) -> &RenderQueue {
        &self.render_queue
    }

    pub fn ui_render_queue(&self) -> &RenderQueue {
        &self.ui_render_queue
    }
}
