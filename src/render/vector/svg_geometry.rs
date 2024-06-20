use std::fs;

use glam::Vec2;
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator,
    TessellationError, VertexBuffers,
};
use usvg::Rect;

const TOLERANCE: f32 = 0.01;

use super::svg_convert::{convert_fill, convert_path, convert_stroke};

#[derive(Debug)]
pub struct SVGGeometry {
    pub name: String,
    pub vertex_buffers: VertexBuffers<Vec2, u32>,
    pub hit_box: Rect,
}

pub struct TesselationOptions {
    pub fill: FillOptions,
    pub stroke: StrokeOptions,
}

impl SVGGeometry {
    pub fn load_svg_from_str(text: &str, name: &str) -> Result<SVGGeometry, Error> {
        let svg = usvg::Tree::from_str(text, &usvg::Options::default())?;

        let options = TesselationOptions {
            fill: FillOptions::default().with_tolerance(TOLERANCE),
            stroke: StrokeOptions::default().with_tolerance(TOLERANCE),
        };

        let center_offset = -Vec2::new(svg.size().width(), svg.size().height()) / 2.0;
        let scale = Vec2::new(1.0, 1.0) / 32.0;

        let mut vertex_buffers = VertexBuffers::new();

        let hit_box = svg
            .node_by_id("hit_box")
            .map(|it| it.abs_bounding_box())
            .unwrap_or(svg.root().abs_bounding_box());

        Self::tesselate(
            svg.root(),
            &mut vertex_buffers,
            &options,
            center_offset,
            scale,
        )?;

        Ok(SVGGeometry {
            name: name.to_string(),
            vertex_buffers,
            hit_box,
        })
    }

    pub fn load_svg_form_path(path: &str, name: Option<&str>) -> Result<SVGGeometry, Error> {
        let svg_text = fs::read_to_string(path)?;
        Self::load_svg_from_str(&svg_text, name.unwrap_or(path))
    }

    fn tesselate(
        svg: &usvg::Group,
        buffers: &mut VertexBuffers<Vec2, u32>,
        options: &TesselationOptions,
        offset: Vec2,
        scale: Vec2,
    ) -> Result<(), TessellationError> {
        for child in svg.children() {
            match child {
                usvg::Node::Group(group) => {
                    Self::tesselate(group, buffers, options, offset, scale)?
                }
                usvg::Node::Path(path) => {
                    Self::tesselate_path_stroke(path, buffers, options, offset, scale)?;
                    Self::tesselate_path_fill(path, buffers, options, offset, scale)?
                }
                usvg::Node::Image(_) => unimplemented!("Image nodes"),
                usvg::Node::Text(_) => unimplemented!("Text nodes"),
            }
        }

        Ok(())
    }

    fn tesselate_path_stroke(
        p: &usvg::Path,
        buffers: &mut VertexBuffers<Vec2, u32>,
        options: &TesselationOptions,
        offset: Vec2,
        scale: Vec2,
    ) -> Result<(), TessellationError> {
        let Some(stroke) = p.stroke() else {
            return Ok(());
        };

        let mut tessellator = StrokeTessellator::new();
        let options = convert_stroke(stroke, options.stroke).1;

        tessellator.tessellate(
            convert_path(p),
            &options,
            &mut BuffersBuilder::new(buffers, |vertex: lyon::tessellation::StrokeVertex| {
                (Vec2::new(vertex.position().x, vertex.position().y) + offset) * scale
            }),
        )
    }

    fn tesselate_path_fill(
        p: &usvg::Path,
        buffers: &mut VertexBuffers<Vec2, u32>,
        options: &TesselationOptions,
        offset: Vec2,
        scale: Vec2,
    ) -> Result<(), TessellationError> {
        let Some(fill) = p.fill() else {
            return Ok(());
        };

        let mut tessellator = FillTessellator::new();
        let options = convert_fill(fill, options.fill).1;

        tessellator.tessellate(
            convert_path(p),
            &options,
            &mut BuffersBuilder::new(buffers, |vertex: lyon::tessellation::FillVertex| {
                (Vec2::new(vertex.position().x, vertex.position().y) + offset) * scale
            }),
        )
    }
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Svg(usvg::Error),
    Tessellation(TessellationError),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}
impl From<usvg::Error> for Error {
    fn from(error: usvg::Error) -> Self {
        Error::Svg(error)
    }
}
impl From<TessellationError> for Error {
    fn from(error: TessellationError) -> Self {
        Error::Tessellation(error)
    }
}
