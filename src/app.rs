use pollster::FutureExt;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
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
            WindowEvent::RedrawRequested => self.render_state.render(),
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        };
    }
}
