use std::collections::HashMap;

use wgpu::{Device, Queue};

use crate::render::msdf::sprite::sprite_sheet::{Sprite, SpriteDef, SpriteSheet};

use super::Manifest;

pub struct MsdfFont {
    pub manifest: Manifest,
    pub sprite_sheet: SpriteSheet,
}

pub struct MsdfFontReference {
    pub manifest: Manifest,
    pub sprite_lookup: HashMap<String, Sprite>,
}

impl MsdfFont {
    pub fn reference(&self) -> MsdfFontReference {
        MsdfFontReference {
            manifest: self.manifest.clone(),
            sprite_lookup: self.sprite_sheet.sprites.clone(),
        }
    }

    pub fn create(device: &Device, queue: &Queue, manifest: &'static str, image: &[u8]) -> Self {
        let manifest = serde_json::from_str::<Manifest>(manifest).unwrap();
        let sprite_sheet = SpriteSheet::create(device, queue, &manifest.clone().into(), image);

        Self {
            manifest,
            sprite_sheet,
        }
    }
}
