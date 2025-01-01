use glam::Vec2;

use crate::{logic::gate::Gate, render::frame::Frame};

use super::GameState;

impl GameState {
    pub fn update_ui(&mut self, frame: &mut Frame) {
        let offset = Vec2::new(64.0, 32.0);
        let button_width = 128.0;
        let buttons = [
            ("AND", Gate::And),
            ("OR", Gate::Or),
            ("NOT", Gate::Not),
            ("NAND", Gate::Nand),
            ("NOR", Gate::Nor),
            ("XOR", Gate::Xor),
            ("XNOR", Gate::Xnor),
            ("BUF", Gate::Buf),
            ("BUTTON", Gate::Button(false)),
            ("IN", Gate::Input(None)),
            ("OUT", Gate::Output(None)),
        ];

        for (index, (name, gate)) in buttons.iter().enumerate() {
            let button_pos = Vec2::new(index as f32 * button_width, 0.0) + offset;

            if frame.button(name, button_pos).clicked {
                self.circuit
                    .circuit
                    .add_gate(gate.clone(), frame.camera().center);
            }
        }
    }
}
