use glam::{Vec2, Vec4};

use crate::{logic::circuit::ConnectionDotRefType, render::frame::Frame};

use super::{input::InputState, GameState};

impl GameState {
    pub fn update(&mut self, frame: &mut Frame) {
        self.handle_inputs(frame.input());

        self.text_object.position = frame.camera().top_left();
        self.text_object.scale = self.camera.size.length() / 30.0;
        self.text_object.position += Vec2::new(0.0, self.text_object.scale);

        self.draw(frame);
    }

    pub fn draw(&self, frame: &mut Frame) {
        for line in self.get_line_instances() {
            frame.draw_line(line);
        }

        self.text_object.draw(frame, &self.font);

        for instance in self.circuit.sprite_instances(&self.sprites) {
            frame.draw_sprite_instance(instance);
        }

        {
            let sprite = self.sprites.get_sprite("dot", "dot").unwrap();
            for dot in self.circuit.connection_dots() {
                let hovering = dot.position.distance(frame.input().mouse_world_position) < 0.0;

                let color = match (dot.ty, hovering) {
                    (ConnectionDotRefType::Input, false) => Vec4::new(1.0, 0.0, 0.0, 1.0),
                    (ConnectionDotRefType::Output, false) => Vec4::new(0.0, 1.0, 0.0, 1.0),
                    (_, true) => Vec4::new(0.0, 0.0, 1.0, 1.0),
                };

                let sprite_instance = sprite.instantiate_with_color(dot.position, 1.0, color);
                frame.draw_sprite_instance(sprite_instance);
            }
        }

        let sprite = self
            .sprites
            .get_sprite("dot", "dot")
            .unwrap()
            .instantiate_with_color(
                frame.input().mouse_world_position,
                1.0,
                Vec4::new(1.0, 1.0, 1.0, 1.0),
            );
        frame.draw_sprite_instance(sprite);
    }

    pub fn handle_inputs(&mut self, input_state: &InputState) {
        if input_state.scroll_delta != 0.0 {
            self.camera
                .scale(Vec2::splat(1.0 + input_state.scroll_delta * 0.1));
        }

        if input_state.right_mouse.down {
            self.camera
                .translate(-input_state.mouse_world_position_delta);
        }
    }
}
