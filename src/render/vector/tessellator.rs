use super::svg_geometry::{self, SVGGeometry};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, MappedMutexGuard, Mutex, MutexGuard},
};

pub static GLOBAL_TESSELLATOR: LazyLock<Tessellator> = LazyLock::new(|| Tessellator {
    settings: TessellationSettings { tolerance: 0.01 },
    vectors: Arc::new(Mutex::new(HashMap::new())),
});

#[derive(Clone, Copy)]
pub struct TessellationSettings {
    pub tolerance: f32,
}

pub struct Tessellator {
    settings: TessellationSettings,
    vectors: Arc<Mutex<HashMap<String, SVGGeometry>>>,
}

impl Tessellator {
    fn tesselate(source: &str) -> SVGGeometry {
        svg_geometry::SVGGeometry::load_svg_from_str(
            source,
            svg_geometry::SVGLoadOptions::default(),
        )
        .unwrap()
    }

    // Internal API to facilitate extracting individual fields without copy of entire SVGGeometry
    fn lazy_tesselate(&self, source: &str) -> MappedMutexGuard<SVGGeometry> {
        let mut vectors = self.vectors.lock().unwrap();

        {
            if !vectors.contains_key(source) {
                let geometry = Self::tesselate(source);
                vectors.insert(source.to_string(), geometry);
            }
        }

        return MutexGuard::map(vectors, |it| it.get_mut(source).unwrap());
    }

    pub fn get_geometry(&self, source: &str) -> SVGGeometry {
        self.lazy_tesselate(source).clone()
    }
}
