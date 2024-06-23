use glam::{Vec2, Vec4};
use lyon::tessellation::VertexBuffers;

use crate::render::{
    msdf::sprite_renderer::{SpriteHandle, SpriteInstance},
    vector::{lazy_instance::LazyVectorInstance, VectorInstance},
    vertex::VertexUV,
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
        let handle = self.render_queue.enqueue_sprite(sprite);
        self.response(handle)
    }

    pub fn line_geo_buffers(&mut self) -> &mut VertexBuffers<VertexUV, u32> {
        &mut self.render_queue.lines
    }

    pub fn draw_vector(&mut self, instance: VectorInstance) -> Response<VectorInstance> {
        let handle = self.render_queue.enqueue_vector(instance);
        self.response(handle)
    }

    pub fn draw_vector_lazy(
        &mut self,
        source: &'static str,
        transform: Vec2,
        color: Vec4,
        scale: Vec2,
    ) {
        let instance = LazyVectorInstance {
            source,
            transform,
            color,
            scale,
        };

        self.render_queue.enqueue_vector_lazy(instance);
    }
}
