use super::{input::InputState, GameState};
use crate::render::frame::Frame;
use common::profiler::Profiler;
use glam::Vec2;

impl GameState {
    pub fn update(&mut self, frame: &mut Frame, profiler: &mut Profiler) {
        self.handle_inputs(frame.input(), profiler);

        profiler.begin("ui");
        self.update_ui(frame);
        profiler.end("ui");

        profiler.begin("solve");
        self.circuit.circuit.step();
        profiler.end("solve");

        // Built before drawing, so every span closed after this point - `draw`
        // and the whole render phase - shows the previous frame's timing.
        self.text_object.content = self.debug_text(frame, profiler);

        profiler.begin("draw");
        self.draw(frame);
        profiler.end("draw");
    }

    pub fn draw(&self, frame: &mut Frame) {
        self.text_object
            .draw(&mut frame.ui_render_queue, &frame.assets.font);

        self.circuit.draw(frame, &self.input);
    }

    fn handle_inputs(&mut self, input_state: &InputState, profiler: &mut Profiler) {
        self.input.prev = self.input.clone().into();
        self.camera_move(input_state);

        profiler.begin("hit test");
        let hovering = self.circuit.hit_test(input_state.mouse_world_position);
        profiler.end("hit test");

        self.input.hot = hovering;
        if input_state.left_mouse.pressed {
            self.input.active = self.input.hot;
        }
        if input_state.left_mouse.released {
            self.input.active = None;
        }

        profiler.begin("edit");
        self.circuit.handle_inputs(input_state, &mut self.input);
        profiler.end("edit");
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
