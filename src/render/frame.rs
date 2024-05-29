pub mod draw;

use super::{camera::Camera, line::LineGeometry, msdf::sprite::sprite_sheet::SpriteInstance};

// Immediate mode context for a frame
pub struct Frame {
    camera: Camera,
    sprites: Vec<SpriteInstance>,
    lines: Vec<LineGeometry>,
}

impl Frame {
    pub fn new(camera: &Camera) -> Self {
        Self {
            camera: *camera,
            sprites: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn sprites(&self) -> &[SpriteInstance] {
        &self.sprites
    }

    pub fn lines(&self) -> &[LineGeometry] {
        &self.lines
    }
}
