use glam::{Vec2, Vec4};

use crate::{
    logic::{circuit::connection::IOSpecifier, hit_test::HitTestResult},
    render::{frame::Frame, msdf::sprite::sprite_sheet::SpriteInstance},
};

use super::{input::InputState, GameState};

impl SpriteInstance {
    pub fn is_colliding(&self, position: Vec2) -> bool {
        let min = self.position - Vec2::splat(0.5) * self.scale;
        let max = self.position + Vec2::splat(0.5) * self.scale;

        position.x >= min.x && position.x <= max.x && position.y >= min.y && position.y <= max.y
    }
}

impl GameState {
    pub fn update(&mut self, frame: &mut Frame) {
        self.handle_inputs(frame.input());

        self.text_object.position = frame.camera().top_left();
        self.text_object.scale = self.camera.size.length() / 30.0;
        self.text_object.position += Vec2::new(0.0, self.text_object.scale);

        self.draw(frame);
    }

    pub fn draw(&self, frame: &mut Frame) {
        for line in self.circuit.connection_instances() {
            for line in line.as_line_geometries(10, 0.05) {
                frame.draw_line(line);
            }
        }

        self.text_object.draw(frame, &self.font);

        for instance in self.circuit.sprite_instances(&self.sprites) {
            frame.draw_sprite_instance(instance);
        }

        {
            let sprite = self.sprites.get_sprite("dot", "dot").unwrap();
            for dot in self.circuit.connection_dots() {
                let position = self.circuit.io_position(dot);
                let hovering = position.distance(frame.input().mouse_world_position) < 0.0;

                let color = match (dot, hovering) {
                    (IOSpecifier::Input(_), false) => Vec4::new(1.0, 0.0, 0.0, 1.0),
                    (IOSpecifier::Output(_), false) => Vec4::new(0.0, 1.0, 0.0, 1.0),
                    (_, true) => Vec4::new(0.0, 0.0, 1.0, 1.0),
                };

                let sprite_instance = sprite.instantiate_with_color(position, 1.0, color);
                frame.draw_sprite_instance(sprite_instance);
            }
        }
    }

    fn handle_inputs(&mut self, input_state: &InputState) {
        self.camera_move(input_state);

        let hovering = self.circuit.hit_test(input_state.mouse_world_position);

        if input_state.left_mouse.pressed {
            self.active = hovering;
        }

        match (self.active, hovering) {
            (Some(HitTestResult::Element(elm)), _) if input_state.left_mouse.down => {
                self.circuit[elm].position += input_state.mouse_world_position_delta;
            }
            (
                Some(HitTestResult::IO(IOSpecifier::Output(output))),
                Some(HitTestResult::IO(IOSpecifier::Input(input))),
            )
            | (
                Some(HitTestResult::IO(IOSpecifier::Input(input))),
                Some(HitTestResult::IO(IOSpecifier::Output(output))),
            ) if input_state.left_mouse.released => {
                self.circuit.add_connection(output.to(input));
            }
            (_, Some(HitTestResult::IO(spec))) if input_state.right_mouse.pressed => {
                self.circuit.delete_connections(spec)
            }
            _ => {}
        }

        self.hot = hovering;
    }

    fn camera_move(&mut self, input_state: &InputState) {
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
