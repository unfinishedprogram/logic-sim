pub mod game_loop;
pub mod input;
use glam::Vec2;

use crate::{
    logic::{circuit::Circuit, hit_test::HitTestResult},
    render::{
        camera::Camera,
        msdf::{
            sprite_renderer::SpriteRendererReference,
            text::{MsdfFontReference, TextObject},
        },
    },
    util::stopwatch::Stopwatch,
};

pub struct GameState {
    pub text_object: TextObject,
    pub camera: Camera,
    pub sprites: SpriteRendererReference,
    pub font: MsdfFontReference,
    circuit: Circuit,

    pub input: GameInput,

    pub stopwatch: Stopwatch,
}

#[derive(Default)]
pub struct GameInput {
    pub hot: Option<HitTestResult>,
    pub active: Option<HitTestResult>,
}

impl GameState {
    pub fn new(font: MsdfFontReference, sprites: SpriteRendererReference) -> Self {
        let text_object = TextObject {
            content: "".to_string(),
            position: Vec2::new(0.0, -1.0),
            scale: 1.0,
        };

        Self {
            camera: Camera::new(),
            text_object,
            font,
            sprites,
            circuit: Circuit::test_circuit(),
            input: GameInput::default(),
            stopwatch: Stopwatch::default(),
        }
    }

    pub fn debug_text(&self) -> String {
        format!(
            "Hot: {:?}\nActive: {:?}\nFrame time: {:.2}ms",
            self.input.hot,
            self.input.hot,
            self.stopwatch.running_average().as_millis_f32()
        )
    }
}
