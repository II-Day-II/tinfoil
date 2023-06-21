use graphics::graphics::Graphics;
use std::time::{Duration, Instant};
use wgpu::SurfaceError;
pub use winit::event::WindowEvent;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod graphics;
mod resources;

pub trait Application {
    fn update(&mut self, dt: Duration);
    fn input(&mut self, event: &WindowEvent) -> bool;
}

pub struct Renderer<T: Application> {
    title: &'static str,
    user_model: T,
}

impl<T> Renderer<T>
where
    T: Application + 'static,
{
    pub fn new(title: &'static str, model: T) -> Self {
        Self {
            title,
            user_model: model,
        }
    }
    pub fn run(mut self) -> ! {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(self.title)
            .build(&event_loop)
            .expect("Couldn't create a window");
        let mut last_render_time = Instant::now();
        let mut graphics = pollster::block_on(Graphics::new(window));
        event_loop.run(move |event, _window_target, control_flow| {
            match event {
                Event::RedrawRequested(id) if id == graphics.window().id() => {
                    let now = Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    self.user_model.update(dt);
                    match graphics.render() {
                        Ok(_) => {}
                        // reconfigure surface if lost
                        Err(SurfaceError::Lost) => graphics.resize(graphics.size()),
                        // if we're out of memory, just quit to avoid any other issues
                        Err(SurfaceError::OutOfMemory) => {
                            eprintln!("Out of memory! Quitting...");
                            *control_flow = ControlFlow::Exit
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    graphics.window().request_redraw();
                }
                Event::WindowEvent {
                    window_id,
                    ref event,
                } if window_id == graphics.window().id() => {
                    if !self.user_model.input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                graphics.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &&mut so we have to dereference it twice
                                graphics.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestApp {
        state: i32,
    }
    impl Application for TestApp {
        fn update(&mut self, _dt: Duration) {
            self.state += _dt.as_secs() as i32;
        }

        fn input(&mut self, _event: &WindowEvent) -> bool {
            //todo!()
            false
        }
    }
    #[test]
    fn runnable() {
        // this is mostly to see that a program can actually compile with the generic + 'static bullshit I'm trying to pull here
        let app = Renderer::new("test", TestApp { state: 0 });
        app.run();
    }
}
