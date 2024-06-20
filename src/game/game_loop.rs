use glam::Vec2;

use crate::{
    logic::{circuit::connection::IOSpecifier, hit_test::HitTestResult},
    render::{frame::Frame, msdf::sprite_renderer::SpriteInstance},
};

use super::{input::InputState, GameInput, GameState};

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

        self.stopwatch.tick();

        let solver = self.circuit.solver.clone();
        let solver = solver.clone().step(&self.circuit);
        self.circuit.solver = solver;

        self.text_object.position = frame.camera().top_left();
        self.text_object.scale = self.camera.size.length() / 50.0;
        self.text_object.position += Vec2::new(0.0, self.text_object.scale);

        self.text_object.content = self.debug_text();

        self.draw(frame);
    }

    pub fn draw(&self, frame: &mut Frame) {
        self.text_object.draw(frame, &self.font);
        self.circuit.draw(frame, &self.input);
    }

    fn handle_inputs(&mut self, input_state: &InputState) {
        self.camera_move(input_state);

        let hovering = self.circuit.hit_test(input_state.mouse_world_position);

        if input_state.left_mouse.pressed {
            self.input.active = hovering;
        }

        match self.input {
            GameInput {
                active: Some(HitTestResult::Element(elm)),
                ..
            } if input_state.left_mouse.down => {
                self.circuit[elm].position += input_state.mouse_world_position_delta;
            }
            GameInput {
                hot: Some(HitTestResult::IO(IOSpecifier::Output(output))),
                active: Some(HitTestResult::IO(IOSpecifier::Input(input))),
            }
            | GameInput {
                hot: Some(HitTestResult::IO(IOSpecifier::Input(input))),
                active: Some(HitTestResult::IO(IOSpecifier::Output(output))),
            } if input_state.left_mouse.released => {
                self.circuit.add_connection(output.to(input));
            }
            GameInput {
                hot: Some(HitTestResult::IO(spec)),
                ..
            } if input_state.right_mouse.pressed => self.circuit.delete_connections(spec),
            _ => {}
        }

        if input_state.left_mouse.released {
            self.input.active = None;
        }
        self.input.hot = hovering;
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
