use glam::Vec2;
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Manifest {
    pages: Vec<String>,
    chars: Vec<Char>,
    info: Info,
    common: Common,
    #[serde(rename = "distanceField")]
    pub distance_field: DistanceField,
    kernings: Vec<Kerning>,
}

impl Manifest {
    pub fn get_char(&self, c: char) -> &Char {
        self.chars
            .iter()
            .find(|ch| ch.char == c)
            .unwrap_or(self.chars.iter().find(|ch| ch.char == '?').unwrap())
    }

    pub fn uvs_of(&self, char_info: &Char) -> (Vec2, Vec2) {
        let start = Vec2::new(
            char_info.x as f32 / self.common.scale_w as f32,
            char_info.y as f32 / self.common.scale_h as f32,
        );

        let end = Vec2::new(
            (char_info.x + char_info.width) as f32 / self.common.scale_w as f32,
            (char_info.y + char_info.height) as f32 / self.common.scale_h as f32,
        );

        (start, end)
    }
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Char {
    pub id: usize,
    pub index: usize,
    pub char: char,
    pub width: usize,
    pub height: usize,
    pub xoffset: i32,
    pub yoffset: i32,
    pub xadvance: usize,
    pub chnl: usize,
    pub x: usize,
    pub y: usize,
    pub page: usize,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Info {
    face: String,
    size: usize,
    bold: usize,
    italic: usize,
    charset: Vec<char>,
    unicode: usize,
    #[serde(rename = "stretchH")]
    stretch_h: usize,
    smooth: usize,
    aa: usize,
    padding: [usize; 4],
    spacing: [usize; 2],
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Common {
    #[serde(rename = "lineHeight")]
    line_height: usize,
    base: usize,
    #[serde(rename = "scaleW")]
    scale_w: usize,
    #[serde(rename = "scaleH")]
    scale_h: usize,
    pages: usize,
    packed: usize,
    #[serde(rename = "alphaChnl")]
    alpha_chnl: usize,
    #[serde(rename = "redChnl")]
    red_chnl: usize,
    #[serde(rename = "greenChnl")]
    green_chnl: usize,
    #[serde(rename = "blueChnl")]
    blue_chnl: usize,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct DistanceField {
    #[serde(rename = "fieldType")]
    field_type: String,
    #[serde(rename = "distanceRange")]
    pub distance_range: usize,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Kerning {
    first: usize,
    second: usize,
    amount: i32,
}
