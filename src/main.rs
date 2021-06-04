#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra_glm as glm;

use glium::glutin::{self, event_loop::EventLoop, window::WindowBuilder, ContextBuilder};
use glium::Surface;

mod camera;
mod keyboard_handler;
mod setup;
mod world;
use camera::*;
use keyboard_handler::*;
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

    let mut keyboard_handler = KeyboardHandler::new();

    let mut world = setup::world();

    let (mut vertex_buffer, mut index_buffer) = world.vertices_and_indices(&display);
    let program = setup::program(display.clone());

    let mut add_block_indice = 0;

    world.camera.turn((0., 0.));

    event_loop.run(move |event, _, control_flow| {

        if let glutin::event::Event::WindowEvent { event, .. } = event {
            if let glutin::event::WindowEvent::CloseRequested = event {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        } else if let glutin::event::Event::DeviceEvent { event, ..} = event {
            if let glutin::event::DeviceEvent::MouseMotion {delta} = event {
                world.camera.turn((delta.0 as f32, delta.1 as f32));
            }
            if let glutin::event::DeviceEvent::Key(input) = event {
                keyboard_handler.process_input(input.scancode);
                keyboard_handler.process_with(|scancode| match scancode {
                    17 => world.camera.move_front(),  // 'z' on azerty
                    31 => world.camera.move_back(), // s 
                    42 => world.camera.move_down(), // Shift
                    57 => world.camera.move_up(), // Space
                    32 => world.camera.move_right(), // d
                    30 => world.camera.move_left(), // q

                    1 => *control_flow = glutin::event_loop::ControlFlow::Exit, // escape

                    16 => {
                        world.add_shape(setup::cube(
                            glm::vec3(
                                (-add_block_indice % 15) as f32,
                                (add_block_indice / 15) as f32,
                                2.,
                            ),
                            (2., 41.),
                            (24., 42.),
                        ));
                        let temp = world.vertices_and_indices(&display);
                        vertex_buffer = temp.0;
                        index_buffer = temp.1;
                        add_block_indice += 1;
                    }
                    _ => (),
                })
            }
        } else if let glutin::event::Event::MainEventsCleared = event {

            let mut target = display.draw();

            let model = glm::identity::<f32, 4>();
            let model: [[f32; 4]; 4] = model.into();

            let view = glm::look_at(&world.camera.camera_pos, &(world.camera.camera_pos + world.camera.camera_front), &glm::vec3(0f32, 1., 0.));
            let view: [[f32; 4]; 4] = view.into();

            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;

            let projection = glm::perspective(aspect_ratio, degree_to_radian(70.), 0.1, 1000.);
            let projection: [[f32; 4]; 4] = projection.into();

            target.clear_color_and_depth((0.6, 0.9, 0.9, 1.0), 1.0); // Sky color
            target
                .draw(
                    &vertex_buffer,
                    &index_buffer,
                    &program,
                    &uniform! {model: model, view: view, projection: projection, tex: glium::uniforms::Sampler::new(&texture)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)},
                    &setup::draw_param(),
                )
                .unwrap();
            target.finish().unwrap();
        }
    });
}
