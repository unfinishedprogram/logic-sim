use glam::Vec2;

use super::{input::InputState, GameState};

impl GameState {
    pub fn update(&mut self, input_state: &InputState) {
        self.handle_inputs(input_state);
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
