use glam::Vec2;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, keyboard::{self, NamedKey}, window::Window
};

use crate::render::RenderState;

pub struct App<'a> {
    window: &'a Window,
    render_state: RenderState<'a>,
}

impl<'a> App<'a> {
    pub async fn create(window: &'a Window) -> Self {
        let render_state = RenderState::create(window).await;

        Self {
            window,
            render_state,
        }
    }

    pub async fn run(&mut self, event_loop: EventLoop<()>) {
        event_loop.run_app(self).expect("Failure in event loop");
    }

    pub fn update(&mut self) {
        self.render_state.update_camera();
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(new_size) => self.render_state.resize(self.window, new_size),
            WindowEvent::RedrawRequested => {
                println!("Redraw requested");
                self.update();
                self.render_state.render();
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                if let keyboard::Key::Named(key) = event.logical_key {
                    match key {
                        NamedKey::Escape => event_loop.exit(),
                        NamedKey::ArrowLeft => self.render_state.camera.translate(Vec2::new(-1.0, 0.0)),
                        NamedKey::ArrowRight => self.render_state.camera.translate(Vec2::new(1.0, 0.0)),
                        NamedKey::ArrowUp => self.render_state.camera.translate(Vec2::new(0.0, 1.0)),
                        NamedKey::ArrowDown => self.render_state.camera.translate(Vec2::new(0.0, -1.0)),
                        _ => {}
                    }
                    self.window.request_redraw();
                } 
            }
            _ => {}
        };
    }
}
