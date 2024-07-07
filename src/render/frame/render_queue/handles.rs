use std::ops::{Index, IndexMut};

use crate::render::{msdf::sprite_renderer::SpriteInstance, vector::VectorInstance};
use util::handle::Handle;

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

impl Index<Handle<VectorInstance>> for RenderQueue {
    type Output = VectorInstance;

    fn index(&self, handle: Handle<VectorInstance>) -> &Self::Output {
        &self.vector_instances[handle.index]
    }
}

impl IndexMut<Handle<VectorInstance>> for RenderQueue {
    fn index_mut(&mut self, handle: Handle<VectorInstance>) -> &mut Self::Output {
        &mut self.vector_instances[handle.index]
    }
}
