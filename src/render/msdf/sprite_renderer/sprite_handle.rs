use glam::{Vec2, Vec4};

use crate::render::msdf::sprite::sprite_sheet::SpriteInstance;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpriteHandle {
    pub sheet_idx: usize,
    pub sprite_idx: usize,
}

impl SpriteHandle {
    pub fn instantiate(self, position: Vec2, scale: f32) -> SpriteInstance {
        self.instantiate_with_color(position, scale, Vec4::splat(1.0))
    }

    pub fn instantiate_with_color(self, position: Vec2, scale: f32, color: Vec4) -> SpriteInstance {
        SpriteInstance {
            sprite_handle: self,
            position,
            scale,
            color,
        }
    }
}
