extern crate glium;

use glium::Surface;

use glium::glutin::{self, event_loop::EventLoop, window::WindowBuilder, ContextBuilder};

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("OpenGl in Rust");
    let context_builder = ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.6, 0.9, 0.9, 1.0);
        target.finish().unwrap();

        if let glutin::event::Event::WindowEvent { event, .. } = event {
            if let glutin::event::WindowEvent::CloseRequested = event {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        }
    });
}
