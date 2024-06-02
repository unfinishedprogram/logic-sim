use std::ops::IndexMut;

use glam::Vec4;

use crate::{render::msdf::sprite::sprite_sheet::SpriteInstance, util::handle::Handle};

use super::Frame;

pub struct Response<'a, T> {
    frame: &'a mut Frame,
    handle: Handle<T>,
}

impl Frame {
    pub fn response<T>(&mut self, handle: Handle<T>) -> Response<T> {
        Response {
            frame: self,
            handle,
        }
    }
}

impl<'a, T> Response<'a, T>
where
    Frame: IndexMut<Handle<T>, Output = T>,
{
    pub fn item(&mut self) -> &mut T {
        &mut self.frame[self.handle]
    }
}

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
