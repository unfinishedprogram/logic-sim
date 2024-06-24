use super::Response;
use crate::render::msdf::sprite_renderer::SpriteInstance;

use glam::Vec4;

impl<'a> Response<'a, SpriteInstance> {
    pub fn color(mut self, color: Vec4) -> Self {
        self.item_mut().color = color;
        self
    }
}
