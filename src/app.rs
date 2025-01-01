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
    render::{
        camera::Camera,
        frame::{Frame, FrameAssets},
        RenderState,
    },
};

use common::stopwatch::Stopwatch;

pub struct App<'a> {
    window: &'a Window,
    render_state: RenderState<'a>,
    mouse_position: PhysicalPosition<f64>,
    input: InputState,
    game_state: GameState,
    frame_time: Stopwatch,
}

impl<'a> App<'a> {
    pub async fn create(window: &'a Window) -> Self {
        let render_state = RenderState::create(window).await;
        let input = InputState::default();

        let mut game_state = GameState::new();

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
        let ui_cam = Camera {
            center: self.screen_size() / 2.0,
            size: self.screen_size() / 2.0,
        };

        let frame_assets = FrameAssets {
            sprites: self.render_state.sprite_renderer.reference(),
            vectors: self.render_state.vector_renderer.reference(),
            font: self.render_state.msdf_font_ref.clone(),
        };

        let mut frame = Frame::new(
            self.game_state.camera,
            ui_cam,
            &self.input,
            frame_assets,
            self.screen_size(),
        );
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

                let frame = self.update();

                self.game_state.text_object.content = format!(
                    "Frame_MS: {:}",
                    self.frame_time.running_average().as_millis_f64()
                );

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
                let screen_position = Vec2::new(position.x as f32, position.y as f32);
                let world_position = mouse_world_position(
                    screen_position,
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

                self.input.on_mouse_move(
                    world_position,
                    world_delta,
                    screen_position,
                    screen_delta,
                );
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if matches!(event.logical_key, keyboard::Key::Named(NamedKey::Escape)) {
                    event_loop.exit()
                }
                self.input
                    .on_keyboard_button(event.logical_key, event.state);
            }
            _ => {}
        };
    }
}

pub fn mouse_world_position(mouse_position: Vec2, screen_size: Vec2, camera: &Camera) -> Vec2 {
    let screen_pos_pixels = mouse_position;
    let screen_pos = screen_pos_pixels / screen_size;
    let screen_clip_pos = (screen_pos - 0.5) * 2.0;
    (screen_clip_pos * camera.size) + camera.center
}
