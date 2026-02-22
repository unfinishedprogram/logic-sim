use assets::SVGSource;

use super::svg_geometry::{self, SVGGeometry, TesselationOptions};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, MappedMutexGuard, Mutex, MutexGuard},
};

pub static GLOBAL_TESSELLATOR: LazyLock<Tessellator> =
    LazyLock::new(|| Tessellator(Arc::new(Mutex::new(TessellatorInner::default()))));

#[derive(Clone, Copy)]
pub struct TessellationSettings {
    pub tolerance: f32,
}

pub struct Tessellator(Arc<Mutex<TessellatorInner>>);

#[derive(Default)]
struct TessellatorInner {
    settings: TessellationSettings,
    vectors: HashMap<SVGSource, SVGGeometry>,
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

impl Tessellator {
    fn tesselate(source: &SVGSource, settings: TessellationSettings) -> SVGGeometry {
        svg_geometry::SVGGeometry::load_svg_from_str(source, settings.into()).unwrap()
    }

    // Internal API to facilitate extracting individual fields without copy of entire SVGGeometry
    fn lazy_tesselate(&self, source: &SVGSource) -> MappedMutexGuard<'_, SVGGeometry> {
        let mut inner = self.0.lock().unwrap();

        {
            if !inner.vectors.contains_key(source) {
                let geometry = Self::tesselate(source, inner.settings);
                inner.vectors.insert(source.clone(), geometry);
            }
        }

        MutexGuard::map(inner, |it| it.vectors.get_mut(source).unwrap())
    }

    pub fn get_geometry(&self, source: &SVGSource) -> SVGGeometry {
        self.lazy_tesselate(source).clone()
    }

    pub fn set_settings(&self, settings: TessellationSettings) {
        let mut inner = self.0.lock().unwrap();
        inner.vectors.clear();
        inner.settings = settings;
    }
}
