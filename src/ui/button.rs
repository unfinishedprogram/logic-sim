use glam::{Vec2, Vec4};

use crate::{
    assets,
    render::{frame::Frame, msdf::text::TextObject, vector::lazy_instance::LazyVectorInstance},
    util::bounds::Bounds,
};

pub struct ButtonResponse {
    pub clicked: bool,
    pub hovered: bool,
}

impl Frame {
    pub fn button(&mut self, text: &str, position: Vec2) -> ButtonResponse {
        let scale = Vec2::splat(32.0);
        let bounds = Bounds::new(position, position + scale);
        let (hovered, clicked) = if bounds.contains(self.input().mouse_screen_position) {
            (true, self.input().left_mouse.pressed)
        } else {
            (false, false)
        };

        self.ui_render_queue
            .enqueue_vector_lazy(LazyVectorInstance {
                source: &assets::svg::ui::BUTTON,
                transform: position,
                color: Vec4::ONE,
                scale: scale * 2.0,
                z_index: 0,
            });

        let text_object = TextObject {
            content: text.to_string(),
            position,
            scale: scale.x,
            centered: true,
        };

        text_object.draw(&mut self.ui_render_queue, &self.assets.font);

        ButtonResponse { clicked, hovered }
    }
}
