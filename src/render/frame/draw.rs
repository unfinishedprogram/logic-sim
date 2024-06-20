use glam::{Vec2, Vec4};
use lyon::tessellation::VertexBuffers;

use crate::{
    render::{
        msdf::sprite_renderer::{SpriteHandle, SpriteInstance},
        vector::VectorInstance,
        vertex::VertexUV,
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
        let instance_index = self.render_queue.sprites.len();
        self.render_queue.sprites.push(sprite);
        self.response(Handle::new(instance_index))
    }

    pub fn line_geo_buffers(&mut self) -> &mut VertexBuffers<VertexUV, u32> {
        &mut self.render_queue.lines
    }

    pub fn draw_vector(&mut self, instance: VectorInstance) {
        self.render_queue.vector_instances.push(instance)
    }
}
