#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;

use glium::glutin::{self, event_loop::EventLoop, window::WindowBuilder, ContextBuilder};
use glium::Surface;

mod world;
use world::*;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("OpenGl in Rust");
    let context_builder = ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let shape = Shape::from_vertices(vec![
        Vertex::new([-0.5, -0.5]),
        Vertex::new([0.0, 0.5]),
        Vertex::new([0.5, -0.25]),
    ]);
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape.vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = default_program(display.clone());

    let mut t = -0.5;

    event_loop.run(move |event, _, control_flow| {
        t += 0.001;
        if t > 0.5 {
            t = -0.5;
        }

        let matrix = glm::translate(&glm::identity(), &glm::vec3(t, 0., 0.));
        let matrix: [[f32; 4]; 4] = matrix.into();

        let mut target = display.draw();
        target.clear_color(0.6, 0.9, 0.9, 1.0); // Sky color
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform! {matrix: matrix},
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        if let glutin::event::Event::WindowEvent { event, .. } = event {
            if let glutin::event::WindowEvent::CloseRequested = event {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        }
    });
}
