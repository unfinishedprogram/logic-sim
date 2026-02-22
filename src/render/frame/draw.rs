use assets::SVGSource;
use common::handle::Handle;
use glam::{Vec2, Vec4};

use crate::render::{
    line::cubic_bezier::CubicBezier,
    msdf::sprite_renderer::{SpriteHandle, SpriteInstance},
    vector::lazy_instance::LazyVectorInstance,
};

pub struct CubicBezierInstance {
    pub bezier: CubicBezier,
    pub color: Vec4,
    pub width: f32,
}

use super::Frame;

impl Frame {
    pub fn draw_sprite(
        &mut self,
        sprite_handle: SpriteHandle,
        position: Vec2,
        scale: f32,
    ) -> Handle<SpriteInstance> {
        self.render_queue.enqueue_sprite(SpriteInstance {
            sprite_handle,
            position,
            scale,
            color: Vec4::splat(1.0),
        })
    }

    pub fn draw_vector_lazy(
        &mut self,
        source: &'static SVGSource,
        transform: Vec2,
        color: Vec4,
        scale: Vec2,
        z_index: u16,
    ) {
        let instance = LazyVectorInstance {
            source,
            transform,
            color,
            scale,
            z_index,
        };

        self.render_queue.enqueue_vector_lazy(instance);
    }

    pub fn draw_cubic_bezier(&mut self, curve: CubicBezier, color: Vec4, width: f32) {
        self.render_queue.enqueue_cubic_bezier(CubicBezierInstance {
            bezier: curve,
            color,
            width,
        });
    }
}
