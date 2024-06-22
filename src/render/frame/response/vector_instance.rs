use glam::{Vec2, Vec4};

use super::Response;
use crate::render::vector::VectorInstance;

impl<'a> Response<'a, VectorInstance> {
    pub fn on_click(self, on_click: impl FnOnce(Self) -> Self) -> Self {
        let bounds = self.frame.assets.vectors.hit_boxes[&self.item().id]
            .scale(self.item().scale / 2.0)
            .translate(self.item().transform);

        let mouse_position = self.frame.input().mouse_world_position;

        if bounds.contains(mouse_position) {
            on_click(self)
        } else {
            self
        }
    }

    pub fn color(mut self, color: Vec4) -> Self {
        self.item_mut().color = color;
        self
    }

    pub fn scale(mut self, scale: Vec2) -> Self {
        self.item_mut().scale = scale;
        self
    }
}
