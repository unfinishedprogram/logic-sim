use glam::Vec2;
use serde::Deserialize;

use crate::render::msdf::sprite_renderer::{self};

use crate::render::msdf::sprite_renderer::sheet as sprite_sheet;

#[derive(Deserialize, Clone)]
pub struct Manifest {
    pub name: &'static str,
    pub atlas: Atlas,
    pub glyphs: Vec<Glyph>,
}

#[derive(Deserialize, Clone)]
pub struct Glyph {
    pub unicode: u32,
    pub advance: f32,
    #[serde(rename = "planeBounds")]
    pub plane_bounds: Option<sprite_sheet::Bounds>,
    #[serde(rename = "atlasBounds")]
    pub atlas_bounds: Option<sprite_sheet::Bounds>,
}

#[derive(Deserialize, Clone)]
pub struct Atlas {
    #[serde(rename = "distanceRange")]
    pub distance_range: f32,
    #[serde(rename = "distanceRangeMiddle")]
    pub _distance_range_middle: f32,
    #[serde(rename = "size")]
    pub _size: f32,
    pub width: f32,
    pub height: f32,
    #[serde(rename = "yOrigin")]
    pub _y_origin: String,
}

impl From<&sprite_sheet::Bounds> for (Vec2, Vec2) {
    fn from(val: &sprite_sheet::Bounds) -> Self {
        (
            Vec2::new(val.left, val.top),
            Vec2::new(val.right, val.bottom),
        )
    }
}

impl From<Atlas> for sprite_renderer::sheet::Atlas {
    fn from(val: Atlas) -> Self {
        sprite_sheet::Atlas {
            distance_range: val.distance_range,
            width: val.width,
            height: val.height,
        }
    }
}

impl From<Manifest> for sprite_sheet::Manifest {
    fn from(val: Manifest) -> Self {
        let atlas: sprite_sheet::Atlas = val.atlas.into();
        let sprites = val
            .glyphs
            .iter()
            .map(|glyph| sprite_sheet::SpriteDef {
                name: char::from_u32(glyph.unicode).unwrap().to_string(),
                plane_bounds: glyph.plane_bounds.unwrap_or_default(),
                atlas_bounds: glyph.atlas_bounds.unwrap_or_default(),
            })
            .collect();

        sprite_sheet::Manifest {
            atlas,
            sprites,
            name: val.name,
        }
    }
}
