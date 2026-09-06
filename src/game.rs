pub mod game_loop;
pub mod input;
mod ui;
use glam::Vec2;

use crate::{
    logic::{
        circuit::{Circuit, EditCircuit},
        hit_test::HitTestResult,
    },
    render::{camera::Camera, frame::Frame, msdf::text::TextObject},
};

use common::profiler::Profiler;

pub struct GameState {
    pub text_object: TextObject,
    pub camera: Camera,
    circuit: EditCircuit,

    pub input: GameInput,
}

#[derive(Default, Clone)]
pub struct GameInput {
    pub hot: Option<HitTestResult>,
    pub active: Option<HitTestResult>,
    pub prev: PrevGameInput,
}

impl GameState {
    pub fn new() -> Self {
        let text_object = TextObject {
            content: "".to_string(),
            position: Vec2::new(0.0, 0.0),
            scale: 32.0,
            centered: false,
        };

        Self {
            camera: Camera::new(),
            text_object,
            circuit: Circuit::extreme_test_circuit().into(),
            input: GameInput::default(),
        }
    }

    pub fn debug_text(&self, frame: &Frame, profiler: &Profiler) -> String {
        let controls = "X : Delete\nC : Copy\nV : Paste";
        format!(
            "Hot: {:?}\nActive: {:?}\nDragging: {}\n\n{}\nControls:\n{controls}",
            self.input.hot,
            self.input.active,
            frame.input().dragging(),
            profiler.report(),
        )
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

// Nearly identical to GameInput
// Exists to prevent recursive type
#[derive(Default, Clone)]
pub struct PrevGameInput {
    pub hot: Option<HitTestResult>,
    pub active: Option<HitTestResult>,
}

impl From<GameInput> for PrevGameInput {
    fn from(value: GameInput) -> Self {
        Self {
            hot: value.hot,
            active: value.active,
        }
    }
}
