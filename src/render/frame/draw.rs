use glam::{Vec2, Vec4};
use lyon::tessellation::VertexBuffers;

use crate::{
    render::{
        msdf::{sprite::sprite_sheet::SpriteInstance, sprite_renderer::SpriteHandle},
        vector::{self, VectorObject},
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
        let instance_index = self.sprites.len();
        self.sprites.push(sprite);
        self.response(Handle::new(instance_index))
    }

    pub fn line_geo_buffers(&mut self) -> &mut VertexBuffers<VertexUV, u32> {
        &mut self.lines
    }

    pub fn draw_vector(&mut self, vector_handle: Handle<VectorObject>, position: Vec2) {
        self.draw_vector_with_color(vector_handle, position, Vec4::splat(1.0));
    }

    pub fn draw_vector_with_color(
        &mut self,
        vector_handle: Handle<VectorObject>,
        position: Vec2,
        color: Vec4,
    ) {
        self.vector_instances.push(vector::Instance {
            id: vector_handle,
            transform: position,
            color,
        })
    }
}
