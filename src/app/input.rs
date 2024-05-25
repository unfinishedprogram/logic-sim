use glam::Vec2;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton},
};

use crate::render::camera::{Camera, CameraBinding};

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
        if button == MouseButton::Right {
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

    pub fn mouse_world_position(&self, screen_size: Vec2, camera: &Camera) -> Vec2 {
        let screen_pos_pixels = self.mouse_position;
        let screen_pos = screen_pos_pixels / screen_size;
        let screen_clip_pos = (screen_pos - 0.5) * 2.0;
        let camera_offset = camera.center / Vec2::new(1.0, -1.0);
        (screen_clip_pos * camera.size) + camera_offset
    }
}
