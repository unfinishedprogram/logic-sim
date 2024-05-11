use glam::Vec2;
use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
pub struct Manifest {
    pub atlas: Atlas,
    pub glyphs: Vec<Glyph>,
}

#[derive(Deserialize)]
pub struct Glyph {
    unicode: u32,
    pub advance: f32,
    #[serde(rename = "planeBounds")]
    pub plane_bounds: Option<Bounds>,
    #[serde(rename = "atlasBounds")]
    pub atlas_bounds: Option<Bounds>,
}

#[derive(Deserialize)]
pub struct Atlas {
    #[serde(rename = "distanceRange")]
    pub distance_range: f32,
    #[serde(rename = "distanceRangeMiddle")]
    pub distance_range_middle: f32,
    pub size: f32,
    pub width: f32,
    pub height: f32,
    #[serde(rename = "yOrigin")]
    pub y_origin: String,
}

#[derive(Clone, Copy, Deserialize)]
pub struct Bounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl From<&Bounds> for (Vec2, Vec2) {
    fn from(val: &Bounds) -> Self {
        (
            Vec2::new(val.left, val.top),
            Vec2::new(val.right, val.bottom),
        )
    }
}

impl Manifest {
    pub fn get_glyph(&self, c: char) -> &Glyph {
        self.glyphs
            .iter()
            .find(|ch| ch.unicode == c as u32)
            .unwrap_or(
                self.glyphs
                    .iter()
                    .find(|ch: &&Glyph| ch.unicode == '?' as u32)
                    .unwrap(),
            )
    }

    pub fn uvs_of(&self, atlas_bounds: &Bounds) -> (Vec2, Vec2) {
        let (start, end) = atlas_bounds.into();
        let scale = Vec2::new(self.atlas.width, self.atlas.height);

        (start / scale, end / scale)
    }
}
