mod handles;

use lyon::tessellation::VertexBuffers;

use crate::{
    render::{
        msdf::sprite_renderer::SpriteInstance,
        vector::{lazy_instance::LazyVectorInstance, VectorInstance},
        vertex::VertexUV,
    },
    util::handle::Handle,
};

pub struct RenderQueue {
    pub sprites: Vec<SpriteInstance>,
    pub lines: VertexBuffers<VertexUV, u32>,
    pub vector_instances: Vec<VectorInstance>,
    pub lazy_instances: Vec<LazyVectorInstance<'static>>,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
            lines: VertexBuffers::new(),
            vector_instances: Vec::new(),
            lazy_instances: Vec::new(),
        }
    }

    pub fn enqueue_sprite(&mut self, sprite: SpriteInstance) -> Handle<SpriteInstance> {
        let index = self.sprites.len();
        self.sprites.push(sprite);
        Handle::new(index)
    }

    pub fn enqueue_vector(&mut self, instance: VectorInstance) -> Handle<VectorInstance> {
        let index = self.vector_instances.len();
        self.vector_instances.push(instance);
        Handle::new(index)
    }

    pub fn enqueue_vector_lazy(
        &mut self,
        instance: LazyVectorInstance<'static>,
    ) -> Handle<LazyVectorInstance> {
        let index = self.lazy_instances.len();
        self.lazy_instances.push(instance);
        Handle::new(index)
    }

    pub fn sprites(&self) -> &[SpriteInstance] {
        &self.sprites
    }

    pub fn lines(&self) -> &VertexBuffers<VertexUV, u32> {
        &self.lines
    }

    pub fn vector_instances(&self) -> &[VectorInstance] {
        &self.vector_instances
    }

    pub fn lazy_vector_instances(&self) -> &[LazyVectorInstance] {
        &self.lazy_instances
    }
}

impl Default for RenderQueue {
    fn default() -> Self {
        Self::new()
    }
}
