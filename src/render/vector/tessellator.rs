use assets::SVGSource;

use super::svg_geometry::{self, SVGGeometry, TesselationOptions};

#[derive(Clone, Copy)]
pub struct TessellationSettings {
    pub tolerance: f32,
}

#[derive(Default)]
pub struct Tessellator {
    settings: TessellationSettings,
}

impl Tessellator {
    pub fn tesselate(&mut self, source: &SVGSource) -> SVGGeometry {
        svg_geometry::SVGGeometry::load_svg_from_str(source, self.settings.into()).unwrap()
    }
}

impl Default for TessellationSettings {
    fn default() -> Self {
        Self { tolerance: 0.01 }
    }
}

impl From<TessellationSettings> for TesselationOptions {
    fn from(val: TessellationSettings) -> Self {
        TesselationOptions {
            fill: lyon::tessellation::FillOptions::default().with_tolerance(val.tolerance),
            stroke: lyon::tessellation::StrokeOptions::default().with_tolerance(val.tolerance),
        }
    }
}
