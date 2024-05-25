mod input;

use std::time::Instant;

use glam::Vec2;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    keyboard::{self, NamedKey},
    window::Window,
};

use crate::{game::GameState, render::RenderState};

use self::input::Input;

pub struct App<'a> {
    window: &'a Window,
    render_state: RenderState<'a>,
    input: Input,
    last_frame: Instant,
    game_state: GameState,
}

impl<'a> App<'a> {
    pub async fn create(window: &'a Window) -> Self {
        window.set_transparent(true);
        let render_state = RenderState::create(window).await;
        let input = Input::default();

        let mut game_state = GameState::new(
            render_state.msdf_font.reference(),
            render_state.sprite_renderer.reference(),
        );
        let window_size = window.inner_size();
        let aspect = window_size.width as f32 / window_size.height as f32;
        game_state.camera.set_aspect(aspect, 10.0);

        Self {
            window,
            render_state,
            input,
            last_frame: Instant::now(),
            game_state,
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
            let clip_delta = screen_delta * 2.0 * self.game_state.camera.size;
            self.game_state.camera.translate(clip_delta);
        }

        self.game_state.text_object.position = self
            .input
            .mouse_world_position(self.screen_size(), &self.game_state.camera);

        self.input.update();
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let old_width = self.render_state.base.surface_config.width as f32;
        let old_height = self.render_state.base.surface_config.height as f32;

        let new_width = new_size.width as f32;
        let new_height = new_size.height as f32;

        let scale = (new_width / old_width, new_height / old_height);
        self.game_state.camera.scale(scale.into());

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
                self.render_state
                    .sprite_renderer
                    .update_camera(&self.render_state.base.queue, &self.game_state.camera);
                self.render_state
                    .render(&self.game_state.get_sprite_instances());
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
                self.game_state.camera.scale(Vec2::splat(scale_delta));
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
                        NamedKey::Backspace => {
                            if event.state == ElementState::Pressed {
                                self.game_state.text_object.content.pop();
                            }
                        }
                        NamedKey::Space => {
                            if event.state == ElementState::Pressed {
                                self.game_state.text_object.content.push(' ');
                            }
                        }
                        _ => {}
                    }
                }

                if let keyboard::Key::Character(c) = event.logical_key {
                    if matches!(event.state, ElementState::Pressed) {
                        self.game_state.text_object.content += &c;
                    }
                }
            }
            _ => {}
        };
    }
}
