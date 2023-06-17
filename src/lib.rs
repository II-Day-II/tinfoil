use std::time::{Duration, Instant};
use graphics::Graphics;
use wgpu::SurfaceError;
use winit::{window::{Window, WindowBuilder}, dpi::PhysicalSize, event::{Event, KeyboardInput, ElementState, VirtualKeyCode}, event_loop::{EventLoop, ControlFlow}};
pub use winit::event::WindowEvent;


pub trait Application {
    fn new(window: Window) -> Self;
    fn render(&mut self) -> Result<(), SurfaceError>;
    fn update(&mut self, dt: Duration);
    fn resize(&mut self, new_size: PhysicalSize<u32>);
    fn input(&mut self, event: &WindowEvent) -> bool;
    fn graphics(&self) -> &Graphics;
}

pub mod graphics;
pub fn run<T: Application +'static>(title: &str) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title(title).build(&event_loop).expect("Couldn't create a window");
    let mut app = T::new(window);
    let mut last_render_time = Instant::now();
    event_loop.run(move |event, _window_target, control_flow| {
        match event {
            Event::RedrawRequested(id) if id == app.graphics().window().id() => {
                let now = Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                app.update(dt);
                match app.render() {
                    Ok(_) => {},
                    // reconfigure surface if lost
                    Err(SurfaceError::Lost) => app.resize(app.graphics().size()),
                    // if we're out of memory, just quit to avoid any other issues
                    Err(SurfaceError::OutOfMemory) => {eprintln!("Out of memory! Quitting..."); *control_flow = ControlFlow::Exit},
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::DeviceEvent { device_id, event } => {
                // TODO: potentially handle mouse input here?
            },
            Event::MainEventsCleared => {
                app.graphics().window().request_redraw();
            },
            Event::WindowEvent { window_id, ref event } if window_id == app.graphics().window().id() => if !app.input(event) {
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
                        app.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        app.resize(**new_inner_size);
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestApp {
        graphics: Graphics,
    }
    impl Application for TestApp {
        fn new(window: Window) -> Self {
            Self {
                graphics: pollster::block_on(Graphics::new(window)),
            }
        }
        fn render(&mut self) -> Result<(), SurfaceError> {
            Ok(())
        }

        fn update(&mut self, _dt: Duration) {
            //todo!()
        }

        fn resize(&mut self, _new_size: PhysicalSize<u32>) {
            //todo!()
        }

        fn input(&mut self, _event: &WindowEvent) -> bool {
            //todo!()
            false
        }

        fn graphics(&self) -> &Graphics {
            &self.graphics
        }
    }
    #[test]
    fn runnable() { // this is mostly to see that a program can actually compile with the generic + 'static bullshit I'm trying to pull here
        run::<TestApp>("test");
    }
}