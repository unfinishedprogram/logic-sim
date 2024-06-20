use std::ops::{Index, IndexMut};

use crate::{
    render::{msdf::sprite::sprite_sheet::SpriteInstance, vector},
    util::handle::Handle,
};

use super::RenderQueue;

impl IndexMut<Handle<SpriteInstance>> for RenderQueue {
    fn index_mut(&mut self, handle: Handle<SpriteInstance>) -> &mut Self::Output {
        &mut self.sprites[handle.index]
    }
}

impl Index<Handle<SpriteInstance>> for RenderQueue {
    type Output = SpriteInstance;

    fn index(&self, handle: Handle<SpriteInstance>) -> &Self::Output {
        &self.sprites[handle.index]
    }
}

impl Index<Handle<vector::Instance>> for RenderQueue {
    type Output = vector::Instance;

    fn index(&self, handle: Handle<vector::Instance>) -> &Self::Output {
        &self.vector_instances[handle.index]
    }
}

impl IndexMut<Handle<vector::Instance>> for RenderQueue {
    fn index_mut(&mut self, handle: Handle<vector::Instance>) -> &mut Self::Output {
        &mut self.vector_instances[handle.index]
    }
}
