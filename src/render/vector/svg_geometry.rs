use std::fs;

use assets::SVGSource;
use glam::{Vec2, Vec4};
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator,
    TessellationError, VertexBuffers,
};
use common::bounds::Bounds;

use super::{
    svg_convert::{convert_fill, convert_path, convert_stroke},
    vertex::SVGVertex,
};

#[derive(Clone, Debug)]
pub struct SVGGeometry {
    pub source: SVGSource,
    pub vertex_buffers: VertexBuffers<SVGVertex, u32>,
    pub hit_box: Bounds,
}

pub struct TesselationOptions {
    pub fill: FillOptions,
    pub stroke: StrokeOptions,
}

impl SVGGeometry {
    pub fn load_svg_from_str(
        source: &SVGSource,
        options: TesselationOptions,
    ) -> Result<SVGGeometry, Error> {
        let svg = usvg::Tree::from_str(&source.0, &usvg::Options::default())?;

        let center_offset = -Vec2::new(svg.size().width(), svg.size().height()) / 2.0;
        let scale = Vec2::new(1.0, 1.0) / 32.0;

        let mut vertex_buffers = VertexBuffers::new();

        let b_box = svg.root().abs_bounding_box();
        let hit_box = Bounds::from_points(
            Vec2::new(b_box.left(), b_box.top()),
            Vec2::new(b_box.right(), b_box.bottom()),
        );

        Self::tesselate(
            svg.root(),
            &mut vertex_buffers,
            &options,
            center_offset,
            scale,
        )?;

        Ok(SVGGeometry {
            vertex_buffers,
            hit_box,
            source: source.clone(),
        })
    }

    pub fn load_svg_from_path(
        path: &str,
        options: TesselationOptions,
    ) -> Result<SVGGeometry, Error> {
        let svg_text = fs::read_to_string(path)?;
        Self::load_svg_from_str(&SVGSource(svg_text), options)
    }

    fn tesselate(
        svg: &usvg::Group,
        buffers: &mut VertexBuffers<SVGVertex, u32>,
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

    fn convert_color(c: usvg::Color) -> Vec4 {
        Vec4::new(
            c.red as f32 / 255.0,
            c.green as f32 / 255.0,
            c.blue as f32 / 255.0,
            1.0,
        )
        // Convert color spaces
        .powf(2.2)
    }

    fn tesselate_path_stroke(
        p: &usvg::Path,
        buffers: &mut VertexBuffers<SVGVertex, u32>,
        options: &TesselationOptions,
        offset: Vec2,
        scale: Vec2,
    ) -> Result<(), TessellationError> {
        let Some(stroke) = p.stroke() else {
            return Ok(());
        };

        let mut color = match stroke.paint() {
            usvg::Paint::Color(c) => Self::convert_color(*c),
            _ => return Ok(()),
        };
        color *= stroke.opacity().get();

        let mut tessellator = StrokeTessellator::new();
        let options = convert_stroke(stroke, options.stroke).1;

        tessellator.tessellate(
            convert_path(p),
            &options,
            &mut BuffersBuilder::new(buffers, |vertex: lyon::tessellation::StrokeVertex| {
                SVGVertex::new(
                    (Vec2::new(vertex.position().x, vertex.position().y) + offset) * scale,
                    color,
                )
            }),
        )
    }

    fn tesselate_path_fill(
        p: &usvg::Path,
        buffers: &mut VertexBuffers<SVGVertex, u32>,
        options: &TesselationOptions,
        offset: Vec2,
        scale: Vec2,
    ) -> Result<(), TessellationError> {
        let Some(fill) = p.fill() else {
            return Ok(());
        };

        let mut color = match fill.paint() {
            usvg::Paint::Color(c) => Self::convert_color(*c),
            _ => return Ok(()),
        };
        color *= fill.opacity().get();

        let mut tessellator = FillTessellator::new();
        let options = convert_fill(fill, options.fill).1;

        tessellator.tessellate(
            convert_path(p),
            &options,
            &mut BuffersBuilder::new(buffers, |vertex: lyon::tessellation::FillVertex| {
                SVGVertex::new(
                    (Vec2::new(vertex.position().x, vertex.position().y) + offset) * scale,
                    color,
                )
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
