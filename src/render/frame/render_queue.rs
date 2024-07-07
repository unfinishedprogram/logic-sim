mod handles;

use glam::Vec4;
use lyon::tessellation::VertexBuffers;
use util::handle::Handle;

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::render::{
    line::cubic_bezier::CubicBezier,
    msdf::sprite_renderer::SpriteInstance,
    vector::{
        lazy_instance::LazyVectorInstance, vertex_buffers::VertexBufferUtils, VectorInstance,
    },
    vertex::VertexUV,
};

pub struct CubicBezierRenderRequest {
    pub bezier: CubicBezier,
    pub color: Vec4,
    pub width: f32,
}

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

    pub fn enqueue_cubic_beziers(&mut self, curves: Vec<CubicBezierRenderRequest>, tolerance: f32) {
        let fold = |mut vb, req: &CubicBezierRenderRequest| {
            req.bezier
                .tesselate(&mut vb, req.width, req.color, tolerance);
            vb
        };

        #[cfg(not(feature = "rayon"))]
        let buffers = {
            curves
                .iter()
                .fold(VertexBuffers::<VertexUV, u32>::new(), fold)
        };

        #[cfg(feature = "rayon")]
        let buffers = {
            VertexBufferUtils::join(
                curves
                    .par_iter()
                    .fold_with(VertexBuffers::<VertexUV, u32>::new(), fold)
                    .collect(),
            )
        };

        self.lines.extend(buffers);
    }

    pub fn enqueue_cubic_bezier(&mut self, curve: CubicBezierRenderRequest, tolerance: f32) {
        self.enqueue_cubic_beziers(vec![curve], tolerance);
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
