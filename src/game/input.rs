use glam::Vec2;
use winit::event::{ElementState, MouseButton};

// State passed to the update function of the game
#[derive(Default)]
pub struct InputState {
    pub mouse_world_position: Vec2,
    pub mouse_world_position_delta: Vec2,
    pub left_mouse: ButtonState,
    pub right_mouse: ButtonState,
    pub scroll_delta: f32,
}

#[derive(Default)]
pub struct ButtonState {
    pub pressed: bool,
    pub released: bool,
    pub down: bool,
}

impl InputState {
    pub fn update(&mut self) {
        // Reset the pressed and released states
        self.left_mouse.update();
        self.right_mouse.update();
        self.mouse_world_position_delta = Vec2::ZERO;
        self.scroll_delta = 0.0;
    }

    pub fn on_mouse_move(&mut self, new_mouse_world_pos: Vec2, mouse_delta: Vec2) {
        self.mouse_world_position_delta += mouse_delta;
        self.mouse_world_position = new_mouse_world_pos;
    }

    pub fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        match button {
            MouseButton::Left => match state {
                ElementState::Pressed => {
                    self.left_mouse.pressed = true;
                    self.left_mouse.down = true;
                }
                ElementState::Released => {
                    self.left_mouse.released = true;
                    self.left_mouse.down = false;
                }
            },
            MouseButton::Right => match state {
                ElementState::Pressed => {
                    self.right_mouse.pressed = true;
                    self.right_mouse.down = true;
                }
                ElementState::Released => {
                    self.right_mouse.released = true;
                    self.right_mouse.down = false;
                }
            },
            _ => {}
        }
    }

    pub fn on_scroll(&mut self, delta: f32) {
        self.scroll_delta += delta;
    }
}

impl ButtonState {
    pub fn update(&mut self) {
        self.pressed = false;
        self.released = false;
    }
}
