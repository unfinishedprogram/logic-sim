use glam::{Vec2, Vec4};

use super::SpriteInstance;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpriteSheetHandle {
    pub idx: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SheetSpriteHandle {
    pub idx: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpriteHandle {
    pub sheet: SpriteSheetHandle,
    pub sprite: SheetSpriteHandle,
}

impl SpriteHandle {
    pub fn new(sheet: usize, sprite: usize) -> Self {
        SpriteHandle {
            sheet: SpriteSheetHandle { idx: sheet },
            sprite: SheetSpriteHandle { idx: sprite },
        }
    }

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
