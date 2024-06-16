pub mod draw;
pub mod handle;
pub mod response;

use lyon::tessellation::VertexBuffers;

use crate::game::input::InputState;

use super::{
    camera::Camera,
    msdf::{sprite::sprite_sheet::SpriteInstance, sprite_renderer::SpriteRendererReference},
    vector::{self, VectorRendererReference},
    vertex::VertexUV,
};

// Immediate mode context for a frame
pub struct Frame {
    camera: Camera,
    sprites: Vec<SpriteInstance>,
    lines: VertexBuffers<VertexUV, u32>,
    vector_instances: Vec<vector::Instance>,
    input_state: InputState,
    pub assets: FrameAssets,
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
            sprites: Vec::new(),
            vector_instances: Vec::new(),
            lines: VertexBuffers::new(),
            assets: FrameAssets { sprites, vectors },
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

    pub fn vector_instances(&self) -> &[vector::Instance] {
        &self.vector_instances
    }
}
