#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra_glm as glm;

use glium::glutin::{self, event_loop::EventLoop, window::WindowBuilder, ContextBuilder};
use glium::Surface;

use std::io::Cursor;

mod world;
use world::*;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("OpenGl in Rust");
    let context_builder = ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let image = image::load(
        Cursor::new(&include_bytes!("../assets/textures/minecraft_tex.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let width = 24f32;
    let height = 42f32;

    let block1 = (1f32, 41f32); // Stone

    let shape = Shape::from_vertices(
        vec![
            Vertex::new([-1.0, -0.5, -1.], [block1.0 / width, block1.1 / height]),
            Vertex::new(
                [-1.0, 0.5, -1.],
                [block1.0 / width, (block1.1 + 1.) / height],
            ),
            Vertex::new(
                [-0.5, 0.5, -1.],
                [(block1.0 + 1.) / width, (block1.1 + 1.) / height],
            ),
            Vertex::new(
                [-0.5, -0.5, -1.],
                [(block1.0 + 1.) / width, block1.1 / height],
            ),
        ],
        vec![0, 1, 2, 0, 2, 3],
    );
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &shape.indices,
    )
    .unwrap();
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
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
                &uniform! {matrix: matrix, tex: glium::uniforms::Sampler::new(&texture)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)},
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
