pub mod draw;
pub mod handle;
pub mod response;

use lyon::tessellation::VertexBuffers;

use crate::game::input::InputState;

use super::{
    camera::Camera,
    msdf::{sprite::sprite_sheet::SpriteInstance, sprite_renderer::SpriteRendererReference},
    vertex::VertexUV,
};

// Immediate mode context for a frame
pub struct Frame {
    camera: Camera,
    sprites: Vec<SpriteInstance>,
    lines: VertexBuffers<VertexUV, u32>,
    input_state: InputState,
    pub assets: FrameAssets,
}

pub struct FrameAssets {
    pub sprites: SpriteRendererReference,
}

impl Frame {
    pub fn new(camera: &Camera, input: &InputState, sprites: SpriteRendererReference) -> Self {
        Self {
            input_state: input.clone(),
            camera: *camera,
            sprites: Vec::new(),
            lines: VertexBuffers::new(),
            assets: FrameAssets { sprites },
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn sprites(&self) -> &[SpriteInstance] {
        &self.sprites
    }

    pub fn lines(&self) -> &VertexBuffers<VertexUV, u32> {
        &self.lines
    }

    pub fn input(&self) -> &InputState {
        &self.input_state
    }
}
