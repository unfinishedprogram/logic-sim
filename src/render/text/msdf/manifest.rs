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

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Char {
    id: usize,
    index: usize,
    char: char,
    width: usize,
    height: usize,
    xoffset: i32,
    yoffset: i32,
    xadvance: usize,
    chnl: usize,
    x: usize,
    y: usize,
    page: usize,
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
