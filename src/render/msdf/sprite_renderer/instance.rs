use glam::{Vec2, Vec4};

use crate::{game::clickable::Clickable, render::msdf::sprite_renderer::SpriteHandle};

#[derive(Clone, Copy)]
pub struct SpriteInstance {
    pub sprite_handle: SpriteHandle,
    pub position: Vec2,
    pub scale: f32,
    pub color: Vec4,
}

impl Clickable for SpriteInstance {
    fn hit_test(&self, position: Vec2) -> bool {
        let min = self.position - Vec2::splat(0.5) * self.scale;
        let max = self.position + Vec2::splat(0.5) * self.scale;

        position.x >= min.x && position.x <= max.x && position.y >= min.y && position.y <= max.y
    }
}
