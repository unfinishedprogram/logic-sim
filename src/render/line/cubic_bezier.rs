use crate::render::vertex::VertexUV;

use glam::{Vec2, Vec4};
use lyon::{
    math::point,
    path::Path,
    tessellation::{BuffersBuilder, StrokeOptions, StrokeTessellator, StrokeVertex, VertexBuffers},
};

pub struct CubicBezier {
    pub start: Vec2,
    pub control1: Vec2,
    pub control2: Vec2,
    pub end: Vec2,
}

impl CubicBezier {
    pub fn between_points(start: Vec2, end: Vec2) -> Self {
        let mix_x = (start.x + end.x) / 2.0;

        let control1 = Vec2::new(mix_x, start.y);
        let control2 = Vec2::new(mix_x, end.y);

        Self {
            start,
            control1,
            control2,
            end,
        }
    }

    pub fn tesselate(&self, width: f32, color: Vec4, buffers: &mut VertexBuffers<VertexUV, u32>) {
        let mut path = Path::builder();
        path.begin(point(self.start.x, self.start.y));
        let ctrl1 = point(self.control1.x, self.control1.y);
        let ctrl2 = point(self.control2.x, self.control2.y);
        let end = point(self.end.x, self.end.y);
        path.cubic_bezier_to(ctrl1, ctrl2, end);
        path.end(false);
        let path = path.build();

        let mut tessellator = StrokeTessellator::new();

        let options = StrokeOptions::default()
            .with_line_width(width)
            .with_tolerance(0.01);

        tessellator
            .tessellate_path(
                &path,
                &options,
                &mut BuffersBuilder::new(buffers, |vertex: StrokeVertex| {
                    VertexUV::new(vertex.position().x, vertex.position().y, 0.0, 0.0, color)
                }),
            )
            .unwrap();
    }
}
