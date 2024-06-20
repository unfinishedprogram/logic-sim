use glam::Vec4;

use crate::render::msdf::sprite::sprite_sheet::SpriteInstance;

use super::Response;

impl<'a> Response<'a, SpriteInstance> {
    pub fn on_click(mut self, on_click: impl FnOnce(Self) -> Self) -> Self {
        let mouse_position = self.frame.input().mouse_world_position;
        let is_colliding = self.item().is_colliding(mouse_position);
        if is_colliding {
            on_click(self)
        } else {
            self
        }
    }

    pub fn color(mut self, color: Vec4) -> Self {
        self.item().color = color;
        self
    }
}
