mod input;

use std::time::Instant;

use glam::Vec2;
use winit::{
    application::ApplicationHandler,
    event::{MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    keyboard::{self, NamedKey},
    window::Window,
};

use crate::render::RenderState;

use self::input::Input;

pub struct App<'a> {
    window: &'a Window,
    render_state: RenderState<'a>,
    input: Input,
    last_frame: Instant,
}

impl<'a> App<'a> {
    pub async fn create(window: &'a Window) -> Self {
        let render_state = RenderState::create(window).await;
        let input = Input::default();

        Self {
            window,
            render_state,
            input,
            last_frame: Instant::now(),
        }
    }

    fn screen_size(&self) -> Vec2 {
        Vec2::new(
            self.window.inner_size().width as f32,
            self.window.inner_size().height as f32,
        )
    }

    pub async fn run(&mut self, event_loop: EventLoop<()>) {
        event_loop.run_app(self).expect("Failure in event loop");
    }

    fn update(&mut self) {
        if self.input.drag {
            let screen_delta_pixels = self.input.mouse_delta() * Vec2::new(-1.0, 1.0);
            let screen_delta = screen_delta_pixels / self.screen_size();
            let clip_delta =
                screen_delta * 2.0 * self.render_state.binding_state.camera.uniform.size;
            self.render_state.binding_state.camera.translate(clip_delta);
        }

        self.input.update();
        self.render_state.update_camera();
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.render_state.resize(self.window, new_size);
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll)
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(new_size) => self.resize(new_size),
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                self.last_frame = now;
                self.update();
                self.render_state.render();

                self.window.request_redraw();
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => self.input.handle_mouse_input(state, button),
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::MouseWheel {
                device_id: _,
                delta: MouseScrollDelta::LineDelta(_x, y),
                phase: _,
            } => {
                let sensitivity = 0.1;
                let scale_delta = 1.0 + y * sensitivity;
                self.render_state
                    .binding_state
                    .camera
                    .scale(Vec2::splat(scale_delta));
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.input.handle_mouse_move(position);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if let keyboard::Key::Named(key) = event.logical_key {
                    match key {
                        NamedKey::Escape => event_loop.exit(),
                        NamedKey::ArrowLeft => self
                            .render_state
                            .binding_state
                            .camera
                            .translate(Vec2::new(-1.0, 0.0)),
                        NamedKey::ArrowRight => self
                            .render_state
                            .binding_state
                            .camera
                            .translate(Vec2::new(1.0, 0.0)),
                        NamedKey::ArrowUp => self
                            .render_state
                            .binding_state
                            .camera
                            .translate(Vec2::new(0.0, 1.0)),
                        NamedKey::ArrowDown => self
                            .render_state
                            .binding_state
                            .camera
                            .translate(Vec2::new(0.0, -1.0)),
                        _ => {}
                    }
                }
            }
            _ => {}
        };
    }
}
