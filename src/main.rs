#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra_glm as glm;

use glium::glutin::{self, event_loop::EventLoop, window::WindowBuilder, ContextBuilder};
use glium::Surface;

mod setup;
mod world;
use world::*;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("OpenGl in Rust");
    let context_builder = ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let texture = setup::texture(
        include_bytes!("../assets/textures/minecraft_tex.png"),
        &display,
    );

    let shape = setup::shapes();

    let (vertex_buffer, indices) = shape.indices_and_vertices(&display);
    let program = default_program(display.clone());

    let mut camera_pos = (0f32, 0f32, -1f32);

    event_loop.run(move |event, _, control_flow| {

        if let glutin::event::Event::WindowEvent { event, .. } = event {
            if let glutin::event::WindowEvent::CloseRequested = event {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        } else if let glutin::event::Event::DeviceEvent { event, ..} = event {
            if let glutin::event::DeviceEvent::MouseMotion {delta} = event {
               camera_pos.0 += delta.0 as f32 / 100.;
                camera_pos.1 += delta.1 as f32 / 100.;
            }
            if let glutin::event::DeviceEvent::Key(input) = event {
                camera_pos.2 += match input.scancode {
                    17 => 0.1,  // 'z' on azerty
                    31 => -0.1, // 's' on azerty 
                    _ => 0.,
                }
            }
        } else if let glutin::event::Event::MainEventsCleared = event {
            // Render code moved here
            let model = glm::identity::<f32, 4>();
            let model: [[f32; 4]; 4] = model.into();

            let view = glm::translate(&glm::identity(), &glm::vec3(camera_pos.0, -camera_pos.1, camera_pos.2));
            let view: [[f32; 4]; 4] = view.into();

            let projection = glm::perspective(1., degree_to_radian(70.), 0.1, 1000.);
            let projection: [[f32; 4]; 4] = projection.into();

            let mut target = display.draw();
            target.clear_color(0.6, 0.9, 0.9, 1.0); // Sky color
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniform! {model: model, view: view, projection: projection, tex: glium::uniforms::Sampler::new(&texture)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)},
                    &Default::default(),
                )
                .unwrap();
            target.finish().unwrap();
        }
    });
}
