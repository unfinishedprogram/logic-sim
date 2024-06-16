use std::fs;

use glam::Vec2;
use lyon::tessellation::{
    BuffersBuilder, StrokeOptions, StrokeTessellator, TessellationError, VertexBuffers,
};

use super::svg_convert::{convert_path, convert_stroke};

#[derive(Debug)]
pub struct VectorObject {
    pub name: String,
    pub vertex_buffers: VertexBuffers<Vec2, u32>,
}

impl VectorObject {
    pub fn load_svg_from_str(text: &str, name: &str) -> Result<VectorObject, Error> {
        let svg = usvg::Tree::from_str(text, &usvg::Options::default())?;

        let options = StrokeOptions::default().with_tolerance(0.01);

        let center_offset = Vec2::new(svg.size().width() * -0.5, svg.size().height() * -0.5);
        let scale = Vec2::new(1.0 / svg.size().width(), 1.0 / svg.size().height());

        let mut vertex_buffers = VertexBuffers::new();
        Self::tesselate(
            svg.root(),
            &mut vertex_buffers,
            &options,
            center_offset,
            scale,
        )?;

        Ok(VectorObject {
            name: name.to_string(),
            vertex_buffers,
        })
    }

    pub fn load_svg_form_path(path: &str, name: Option<&str>) -> Result<VectorObject, Error> {
        let svg_text = fs::read_to_string(path)?;
        Self::load_svg_from_str(&svg_text, name.unwrap_or(path))
    }

    fn tesselate(
        svg: &usvg::Group,
        buffers: &mut VertexBuffers<Vec2, u32>,
        options: &StrokeOptions,
        offset: Vec2,
        scale: Vec2,
    ) -> Result<(), TessellationError> {
        for child in svg.children() {
            match child {
                usvg::Node::Group(group) => {
                    Self::tesselate(group, buffers, options, offset, scale)?
                }
                usvg::Node::Path(path) => {
                    Self::tesselate_path(path, buffers, options, offset, scale)?
                }
                usvg::Node::Image(_) => unimplemented!("Image nodes"),
                usvg::Node::Text(_) => unimplemented!("Text nodes"),
            }
        }

        Ok(())
    }

    fn tesselate_path(
        p: &usvg::Path,
        buffers: &mut VertexBuffers<Vec2, u32>,
        options: &StrokeOptions,
        offset: Vec2,
        scale: Vec2,
    ) -> Result<(), TessellationError> {
        // let mut buffers_builder = BuffersBuilder::new(buffers, |pos: Vec2| pos);
        let mut tessellator = StrokeTessellator::new();

        let mut options = *options;
        if let Some(s) = p.stroke() {
            options = convert_stroke(s, options).1
        }

        tessellator.tessellate(
            convert_path(p),
            &options,
            &mut BuffersBuilder::new(buffers, |vertex: lyon::tessellation::StrokeVertex| {
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
