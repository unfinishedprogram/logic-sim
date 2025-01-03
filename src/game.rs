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

use common::stopwatch::Stopwatch;

pub struct GameState {
    pub text_object: TextObject,
    pub camera: Camera,
    circuit: EditCircuit,

    pub input: GameInput,

    pub stopwatch: Stopwatch,
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
            scale: 16.0,
            centered: false,
        };

        Self {
            camera: Camera::new(),
            text_object,
            circuit: Circuit::extreme_test_circuit().into(),
            input: GameInput::default(),
            stopwatch: Stopwatch::default(),
        }
    }

    pub fn debug_text(&self, frame: &Frame) -> String {
        let controls = "\nX : Delete\nC : Copy\nV : Paste\n";
        format!(
            "Hot: {:?}\nActive: {:?}\nFrame time: {:.2}ms\nDragging: {}\n Controls: {controls}",
            self.input.hot,
            self.input.active,
            self.stopwatch.running_average().as_millis_f32(),
            frame.input().dragging()
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
