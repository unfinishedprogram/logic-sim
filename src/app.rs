use glam::Vec2;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    keyboard::{self, NamedKey},
    window::Window,
};

use crate::{
    game::{input::InputState, GameState},
    render::{camera::Camera, frame::Frame, msdf::text::TextObject, RenderState},
    util::stopwatch::Stopwatch,
};

pub struct App<'a> {
    window: &'a Window,
    render_state: RenderState<'a>,
    mouse_position: PhysicalPosition<f64>,
    input: InputState,
    game_state: GameState,
    frame_time: Stopwatch,
}

pub fn mouse_world_position(mouse_position: Vec2, screen_size: Vec2, camera: &Camera) -> Vec2 {
    let screen_pos_pixels = mouse_position;
    let screen_pos = screen_pos_pixels / screen_size;
    let screen_clip_pos = (screen_pos - 0.5) * 2.0;
    (screen_clip_pos * camera.size) + camera.center
}

impl<'a> App<'a> {
    pub async fn create(window: &'a Window) -> Self {
        window.set_transparent(true);
        let render_state = RenderState::create(window).await;
        let input = InputState::default();

        let mut game_state = GameState::new(
            render_state.msdf_font_ref.clone(),
            render_state.sprite_renderer.reference(),
        );

        let window_size = window.inner_size();
        let aspect = window_size.width as f32 / window_size.height as f32;
        game_state.camera.set_aspect(aspect, 10.0);

        Self {
            window,
            render_state,
            input,
            game_state,
            mouse_position: PhysicalPosition::new(0.0, 0.0),
            frame_time: Stopwatch::default(),
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

    fn update(&mut self) -> Frame {
        let mut frame = Frame::new(&self.game_state.camera, &self.input);
        self.input.update();
        self.game_state.update(&mut frame);
        frame
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.game_state
            .camera
            .set_aspect_ratio(Vec2::new(new_size.width as f32, new_size.height as f32));

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
                self.frame_time.tick();

                let mut frame = self.update();

                self.game_state.text_object.content = format!(
                    "Frame_MS: {:}",
                    self.frame_time.running_average().as_millis_f64()
                );

                for i in 0..100 {
                    TextObject {
                        content: format!("Text Object {}", i).repeat(50),
                        position: Vec2::new(0.0, i as f32 * 0.05),
                        scale: 0.05,
                    }
                    .draw(&mut frame, &self.game_state.font)
                }

                self.render_state.render(frame);

                self.window.request_redraw();
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                self.input.on_mouse_button(button, state);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::MouseWheel {
                device_id: _,
                delta: MouseScrollDelta::LineDelta(_x, y),
                phase: _,
            } => {
                self.input.on_scroll(y);
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                let world_position = mouse_world_position(
                    Vec2::new(position.x as f32, position.y as f32),
                    self.screen_size(),
                    &self.game_state.camera,
                );

                let pixel_delta = Vec2::new(
                    position.x as f32 - self.mouse_position.x as f32,
                    position.y as f32 - self.mouse_position.y as f32,
                );

                let screen_delta_pixels = pixel_delta;
                let screen_delta = screen_delta_pixels / self.screen_size();
                let world_delta = screen_delta * 2.0 * self.game_state.camera.size;

                self.mouse_position = position;

                self.input.on_mouse_move(world_position, world_delta);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if let keyboard::Key::Named(key) = event.logical_key {
                    if key == NamedKey::Escape {
                        event_loop.exit()
                    }
                }
            }
            _ => {}
        };
    }
}
