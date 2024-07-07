use crate::render::vertex::VertexUV;

use glam::{Vec2, Vec4};
use lyon::{
    math::point,
    path::Path,
    tessellation::{BuffersBuilder, StrokeOptions, StrokeTessellator, StrokeVertex, VertexBuffers},
};
use util::bounds::Bounds;

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

    pub fn tesselate(
        &self,
        buffers: &mut VertexBuffers<VertexUV, u32>,
        width: f32,
        color: Vec4,
        tolerance: f32,
    ) {
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
            .with_tolerance(tolerance);

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

    pub fn hit_test(&self, point: Vec2, distance: f32) -> bool {
        // Since we always make our control points, between the start and end points,
        // we can assume that the maximum bounds of the curve lie between the start and end points
        let bounds = Bounds::from_points(self.start, self.end).pad(distance);

        if !bounds.contains(point) {
            return false;
        }

        // Binary search to find the closest point on the curve

        let mut t = 0.5;
        let mut step = 0.25;

        for _ in 0..16 {
            let lower = self.point_at(t - step).distance(point);
            let higher = self.point_at(t + step).distance(point);

            if lower > higher {
                t += step;
            } else {
                t -= step;
            }
            step /= 2.0;
        }

        self.point_at(t).distance(point) <= distance
    }

    fn point_at(&self, t: f32) -> Vec2 {
        let t2 = t * t;
        let t3 = t2 * t;

        let inv_t = 1.0 - t;
        let inv_t2 = inv_t * inv_t;
        let inv_t3 = inv_t2 * inv_t;

        self.start * inv_t3
            + (self.control1 * 3.0 * inv_t2 * t)
            + (self.control2 * 3.0 * inv_t * t2)
            + self.end * t3
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::from_points(self.start, self.end)
    }
}
