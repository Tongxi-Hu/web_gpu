use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window,
};

use crate::state::State;

struct App<'a> {
    state: Option<State<'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("web_gpu"))
            .unwrap();
        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = self.state.as_ref().unwrap().window();
        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(new_size) => {
                    self.state.as_mut().unwrap().resize(new_size);
                }
                WindowEvent::RedrawRequested => {
                    self.state.as_mut().unwrap().render().unwrap();
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        let window = self.state.as_ref().unwrap().window();
        window.request_redraw();
    }
}

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let mut app_state = App::new();
    event_loop.run_app(&mut app_state).unwrap();
}
