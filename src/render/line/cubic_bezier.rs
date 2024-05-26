use glam::Vec2;

use super::geometry::LineGeometry;

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

    pub fn as_line_geometries(&self, mut resolution: usize, width: f32) -> Vec<LineGeometry> {
        if resolution < 2 {
            resolution = 2;
        }

        let samples: Vec<(Vec2, Vec2)> = (0..resolution)
            .map(|i| {
                let t = i as f32 / (resolution - 1) as f32;
                self.sample_pair(t, width)
            })
            .collect();

        samples
            .array_windows::<2>()
            .map(|[left, right]| {
                LineGeometry::from_corner_points([left.0, left.1, right.0, right.1])
            })
            .collect()
    }

    // Samples a pair of points, which are equidistant to the curve at time t
    fn sample_pair(&self, t: f32, width: f32) -> (Vec2, Vec2) {
        let p = self.sample(t);
        let n = self.normal(t);

        let offset = n * width / 2.0;

        (p + offset, p - offset)
    }

    fn sample(&self, t: f32) -> Vec2 {
        let t2 = t * t;
        let t3 = t2 * t;
        let u = 1.0 - t;
        let u2 = u * u;
        let u3 = u2 * u;

        let mut p = u3 * self.start;
        p += 3.0 * u2 * t * self.control1;
        p += 3.0 * u * t2 * self.control2;
        p += t3 * self.end;

        p
    }

    fn derivative(&self, t: f32) -> Vec2 {
        let t2 = t * t;
        let u = 1.0 - t;
        let u2 = u * u;

        let mut p = 3.0 * u2 * (self.control1 - self.start);
        p += 6.0 * u * t * (self.control2 - self.control1);
        p += 3.0 * t2 * (self.end - self.control2);

        p
    }

    fn normal(&self, t: f32) -> Vec2 {
        let d = self.derivative(t);
        let n = Vec2::new(-d.y, d.x);

        n.normalize()
    }
}
