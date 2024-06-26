use super::{input::InputState, GameState};
use crate::render::frame::Frame;
use glam::Vec2;

impl GameState {
    pub fn update(&mut self, frame: &mut Frame) {
        self.handle_inputs(frame.input());

        self.stopwatch.tick();

        let solver = self.circuit.solver.clone();
        let solver = solver.clone().step(&self.circuit);
        self.circuit.solver = solver;

        self.text_object.content = self.debug_text();

        self.draw(frame);
    }

    pub fn draw(&self, frame: &mut Frame) {
        self.text_object
            .draw(&mut frame.ui_render_queue, &self.font);

        self.circuit.draw(frame, &self.input);
    }

    fn handle_inputs(&mut self, input_state: &InputState) {
        self.camera_move(input_state);

        let hovering = self.circuit.hit_test(input_state.mouse_world_position);

        if input_state.left_mouse.pressed {
            self.input.active = hovering;
        }

        self.circuit.handle_inputs(input_state, &self.input);

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
