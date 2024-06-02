use glam::{Vec2, Vec4};

use crate::{
    render::{
        line::LineGeometry,
        msdf::{sprite::sprite_sheet::SpriteInstance, sprite_renderer::SpriteHandle},
    },
    util::handle::Handle,
};

use super::{response::Response, Frame};

impl Frame {
    pub fn draw_sprite(
        &mut self,
        sprite_handle: SpriteHandle,
        position: Vec2,
        scale: f32,
    ) -> Response<SpriteInstance> {
        self.draw_sprite_instance(SpriteInstance {
            sprite_handle,
            position,
            scale,
            color: Vec4::splat(1.0),
        })
    }

    pub fn draw_sprite_instance(&mut self, sprite: SpriteInstance) -> Response<SpriteInstance> {
        let instance_index = self.sprites.len();
        self.sprites.push(sprite);
        self.response(Handle::new(instance_index))
    }

    #[inline]
    pub fn draw_line(&mut self, line: LineGeometry) {
        self.lines.push(line);
    }
}
