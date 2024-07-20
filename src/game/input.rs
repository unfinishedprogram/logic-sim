use std::collections::HashMap;

use glam::Vec2;
use winit::{
    event::{ElementState, MouseButton},
    keyboard::Key,
};

const DRAG_THRESHOLD: f32 = 2.0;

// State passed to the update function of the game
#[derive(Default, Clone)]
pub struct InputState {
    pub mouse_world_position: Vec2,
    pub mouse_world_position_delta: Vec2,

    // In camera's screen space
    pub mouse_screen_position: Vec2,
    pub mouse_screen_position_delta: Vec2,

    pub drag_start_position: Option<Vec2>,

    pub left_mouse: ButtonState,
    pub right_mouse: ButtonState,
    pub scroll_delta: f32,

    pub keyboard: KeyboardState,
}

#[derive(Default, Clone, Debug)]
pub struct ButtonState {
    pub pressed: bool,
    pub released: bool,
    pub down: bool,
}

impl ButtonState {
    fn update(&mut self) {
        self.pressed = false;
        self.released = false;
    }

    fn apply(&mut self, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.pressed = true;
                self.down = true;
            }
            ElementState::Released => {
                self.released = true;
                self.down = false;
            }
        }
    }
}

impl InputState {
    pub fn update(&mut self) {
        if !self.left_mouse.down && !self.left_mouse.released {
            self.drag_start_position = None;
        }

        // Reset the pressed and released states
        self.left_mouse.update();
        self.right_mouse.update();
        self.keyboard.update();

        self.mouse_world_position_delta = Vec2::ZERO;
        self.mouse_screen_position_delta = Vec2::ZERO;
        self.scroll_delta = 0.0;
    }

    pub fn dragging(&self) -> bool {
        self.drag_start_position.map_or(false, |start| {
            (self.mouse_screen_position - start).length() > DRAG_THRESHOLD
        })
    }

    pub fn on_mouse_move(
        &mut self,
        new_mouse_world_pos: Vec2,
        mouse_delta: Vec2,
        new_mouse_screen_position: Vec2,
        screen_delta: Vec2,
    ) {
        self.mouse_world_position_delta += mouse_delta;
        self.mouse_world_position = new_mouse_world_pos;

        self.mouse_screen_position_delta += screen_delta;
        self.mouse_screen_position = new_mouse_screen_position;
    }

    pub fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        match button {
            MouseButton::Left => {
                self.left_mouse.apply(state);
                if self.left_mouse.pressed {
                    self.drag_start_position = Some(self.mouse_screen_position);
                }
            }
            MouseButton::Right => self.right_mouse.apply(state),
            _ => {}
        }
    }

    pub fn on_keyboard_button(&mut self, key: Key, state: ElementState) {
        self.keyboard.on_keyboard_button(key, state);
    }

    pub fn on_scroll(&mut self, delta: f32) {
        self.scroll_delta += delta;
    }
}

#[derive(Default, Clone)]
pub struct KeyboardState {
    key_states: HashMap<Key, ButtonState>,
}

impl KeyboardState {
    pub fn pressed(&self, key: Key) -> bool {
        self.key_states
            .get(&key)
            .map_or(false, |state| state.pressed)
    }

    pub fn down(&self, key: Key) -> bool {
        self.key_states.get(&key).map_or(false, |state| state.down)
    }

    pub fn released(&self, key: Key) -> bool {
        self.key_states
            .get(&key)
            .map_or(false, |state| state.released)
    }

    fn update(&mut self) {
        for (_, state) in self.key_states.iter_mut() {
            state.update();
        }

        self.key_states
            .retain(|_, state| state.down || state.released || state.pressed);
    }

    fn on_keyboard_button(&mut self, key: Key, state: ElementState) {
        let key_state = self.key_states.entry(key).or_default();
        key_state.apply(state);
    }
}
