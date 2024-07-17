mod handles;

use lyon::tessellation::VertexBuffers;
use util::handle::Handle;

#[cfg(feature = "debug_draw")]
use glam::Vec4;
#[cfg(feature = "debug_draw")]
use lyon::{
    algorithms::winding,
    geom::{point, Box2D},
    path::Path,
    tessellation::{BuffersBuilder, StrokeOptions, StrokeTessellator, StrokeVertex},
};
#[cfg(feature = "debug_draw")]
use util::bounds::Bounds;

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::render::{
    msdf::sprite_renderer::SpriteInstance,
    vector::{
        lazy_instance::LazyVectorInstance, vertex_buffers::VertexBufferUtils, VectorInstance,
    },
    vertex::VertexUV,
};

use super::draw::CubicBezierInstance;

#[derive(Default)]
pub struct RenderQueue {
    pub sprites: Vec<SpriteInstance>,
    pub lines: VertexBuffers<VertexUV, u32>,
    pub vector_instances: Vec<VectorInstance>,
    pub bezier_instances: Vec<CubicBezierInstance>,
    pub lazy_instances: Vec<LazyVectorInstance<'static>>,
}

impl RenderQueue {
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

    pub fn tesselate_geometry(&mut self, tolerance: f32) {
        // Take the bezier instances out of the queue
        let mut bezier_instances = Vec::new();
        std::mem::swap(&mut bezier_instances, &mut self.bezier_instances);

        self.tesselate_cubic_beziers(bezier_instances, tolerance);
    }

    // Applies tesselation to endued bezier curves
    fn tesselate_cubic_beziers(&mut self, curves: Vec<CubicBezierInstance>, tolerance: f32) {
        let fold = |mut vb, req: &CubicBezierInstance| {
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

    #[cfg(feature = "debug_draw")]
    pub fn debug_render_bounds(&mut self, bounds: Bounds) {
        let mut path = Path::builder();

        let box_2d = Box2D::from_points([
            point(bounds.top_left.x, bounds.top_left.y),
            point(bounds.bottom_right.x, bounds.bottom_right.y),
        ]);

        path.add_rectangle(&box_2d, lyon::path::Winding::Positive);
        let path = path.build();

        let mut tessellator = StrokeTessellator::new();

        let options = StrokeOptions::default()
            .with_line_width(2.0)
            .with_tolerance(0.0001);

        let red = Vec4::new(1.0, 0.0, 0.0, 1.0);
        tessellator
            .tessellate_path(
                &path,
                &options,
                &mut BuffersBuilder::new(&mut self.lines, |vertex: StrokeVertex| {
                    VertexUV::new(vertex.position().x, vertex.position().y, 0.0, 0.0, red)
                }),
            )
            .unwrap();
    }

    pub fn enqueue_cubic_bezier(&mut self, curve: CubicBezierInstance) {
        self.bezier_instances.push(curve);
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
