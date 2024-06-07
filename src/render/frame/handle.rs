use std::ops::{Index, IndexMut};

use crate::{render::msdf::sprite::sprite_sheet::SpriteInstance, util::handle::Handle};

use super::Frame;

impl IndexMut<Handle<SpriteInstance>> for Frame {
    fn index_mut(&mut self, handle: Handle<SpriteInstance>) -> &mut Self::Output {
        &mut self.sprites[handle.index]
    }
}

impl Index<Handle<SpriteInstance>> for Frame {
    type Output = SpriteInstance;

    fn index(&self, handle: Handle<SpriteInstance>) -> &Self::Output {
        &self.sprites[handle.index]
    }
}
