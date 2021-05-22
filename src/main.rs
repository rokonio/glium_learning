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
    let context_builder = ContextBuilder::new().with_vsync(true).with_depth_buffer(24);
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let texture = setup::texture(
        include_bytes!("../assets/textures/minecraft_tex.png"),
        &display,
    );

    let shape = setup::shapes();

    let (vertex_buffer, indices) = shape.indices_and_vertices(&display);
    let program = default_program(display.clone());

    let mut camera_pos = glm::vec3(0f32, 0., 0.);
    let mut camera_rot = glm::vec3(0f32, 0., 0.);

    event_loop.run(move |event, _, control_flow| {

        if let glutin::event::Event::WindowEvent { event, .. } = event {
            if let glutin::event::WindowEvent::CloseRequested = event {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        } else if let glutin::event::Event::DeviceEvent { event, ..} = event {
            if let glutin::event::DeviceEvent::MouseMotion {delta} = event {
               camera_rot.y += delta.0 as f32 / 100.;
                camera_rot.x += delta.1 as f32 / 100.;
            }
            if let glutin::event::DeviceEvent::Key(input) = event {
                match input.scancode {
                    17 => camera_pos.z += 0.1,  // 'z' on azerty
                    31 => camera_pos.z -= 0.1, // 's' on azerty 
                    42 => camera_pos.y += 0.1, // Shift
                    57 => camera_pos.y -= 0.1, // Space
                    32 => camera_pos.x -= 0.1, // D
                    30 => camera_pos.x += 0.1, // Q
                    _ => (),
                }
            }
        } else if let glutin::event::Event::MainEventsCleared = event {
            // Render code moved here

            let mut target = display.draw();

            let model = glm::identity::<f32, 4>();
            let model: [[f32; 4]; 4] = model.into();

            let view = glm::Mat4x4::identity();
            let view = glm::rotate_x(&view, camera_rot.x);
            let view = glm::rotate_y(&view, camera_rot.y);
            let view = glm::translate(&view, &camera_pos);
            let view: [[f32; 4]; 4] = view.into();

            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;

            let projection = glm::perspective(aspect_ratio, degree_to_radian(70.), 0.1, 1000.);
            let projection: [[f32; 4]; 4] = projection.into();

            target.clear_color_and_depth((0.6, 0.9, 0.9, 1.0), 1.0); // Sky color
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniform! {model: model, view: view, projection: projection, tex: glium::uniforms::Sampler::new(&texture)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)},
                    &default_draw_param(),
                )
                .unwrap();
            target.finish().unwrap();
        }
    });
}
