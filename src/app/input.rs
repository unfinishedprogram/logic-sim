use glam::Vec2;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton},
};

#[derive(Default)]
pub struct Input {
    pub drag: bool,
    pub prev_mouse_position: Vec2,
    pub mouse_position: Vec2,
}

impl Input {
    pub fn update(&mut self) {
        self.prev_mouse_position = self.mouse_position;
    }

    pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            match state {
                ElementState::Pressed => {
                    self.drag = true;
                }
                ElementState::Released => {
                    self.drag = false;
                }
            }
        }
    }

    pub fn handle_mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_position = Vec2::new(position.x as f32, position.y as f32);
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_position - self.prev_mouse_position
    }
}
