use glam::{Vec2, Vec4};

use crate::render::msdf::sprite_renderer::SpriteHandle;

#[derive(Clone, Copy)]
pub struct SpriteInstance {
    pub sprite_handle: SpriteHandle,
    pub position: Vec2,
    pub scale: f32,
    pub color: Vec4,
}
