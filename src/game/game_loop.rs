use glam::Vec2;

use crate::render::frame::Frame;

use super::{input::InputState, GameState};

impl GameState {
    pub fn update(&mut self, input_state: &InputState, frame: &mut Frame) {
        self.handle_inputs(input_state);
        self.draw(frame);
    }

    pub fn draw(&self, frame: &mut Frame) {
        for sprite in self.get_sprite_instances() {
            frame.draw_sprite(sprite);
        }

        for line in self.get_line_instances() {
            frame.draw_line(line);
        }
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
