use std::collections::HashMap;

use wgpu::{Device, Queue};

use crate::render::msdf::sprite_renderer::{SpriteHandle, SpriteSheet};

use super::Manifest;

pub struct MsdfFont {
    pub manifest: Manifest,
    pub sprite_sheet: SpriteSheet,
}

const ASCII_RANGE: usize = 128;

#[derive(Clone)]
pub struct MsdfFontReference {
    unicode_lookup: HashMap<char, SpriteHandle>,
    ascii_lookup: [SpriteHandle; ASCII_RANGE],
    advance_lookup: HashMap<char, f32>,
}

impl MsdfFontReference {
    pub fn get(&self, c: char) -> Option<SpriteHandle> {
        if c.is_ascii() {
            Some(self.ascii_lookup[c as usize])
        } else {
            self.unicode_lookup.get(&c).copied()
        }
    }

    pub fn advance(&self, c: char) -> f32 {
        self.advance_lookup.get(&c).copied().unwrap_or(0.0)
    }
}

impl MsdfFont {
    pub fn reference(&self) -> MsdfFontReference {
        let unicode_lookup = self
            .sprite_sheet
            .sprites
            .iter()
            .map(|(k, v)| (k.chars().next().unwrap(), SpriteHandle::new(0, *v)))
            .collect();

        let mut ascii_lookup = [SpriteHandle::new(0, 0); ASCII_RANGE];

        for (name, sprite) in &self.sprite_sheet.sprites {
            if name.len() == 1 {
                let c = name.chars().next().unwrap();
                if c.is_ascii() {
                    ascii_lookup[c as usize] = SpriteHandle::new(0, *sprite);
                }
            }
        }

        let advance_lookup = self
            .manifest
            .glyphs
            .iter()
            .map(|glyph| (char::from_u32(glyph.unicode).unwrap(), glyph.advance))
            .collect();

        MsdfFontReference {
            unicode_lookup,
            ascii_lookup,
            advance_lookup,
        }
    }

    pub fn create(device: &Device, queue: &Queue, manifest: &'static str, image: &[u8]) -> Self {
        let manifest = miniserde::json::from_str::<Manifest>(manifest).unwrap();
        let sprite_sheet = SpriteSheet::create(device, queue, &manifest.clone().into(), image);

        Self {
            manifest,
            sprite_sheet,
        }
    }
}
