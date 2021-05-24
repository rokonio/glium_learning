use super::camera::Camera;
use super::world::*;

use glium;

use std::io::Cursor;

pub fn texture(path: &[u8], display: &glium::Display) -> glium::texture::Texture2d {
    let image = image::load(Cursor::new(path), image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, image).unwrap()
}

pub fn world() -> World {
    let width = 24f32;
    let height = 42f32;

    let stone = (1f32, 41f32);
    let cobble_stone = (0f32, 40f32);
    let grass = (0f32, 41f32);
    let oak_plank = (4f32, 40f32);

    let mut world = World::from_camera(Camera::new(
        0.4,                      // Velocity
        0.4,                      // Sensitivity
        glm::vec3(-0.5, 3., -5.), // Position
        0.,                       // Head x-axis position
        90.,                      // Head y-axis position
    ));

    for x in 0..16 {
        for z in 0..16 {
            for y in 0..32 {
                let block = match (x + z) % 4 {
                    0 => stone,
                    1 => cobble_stone,
                    2 => grass,
                    3 => oak_plank,
                    _ => unreachable!(),
                };
                world.add_shape(cube(
                    glm::vec3(x as f32, y as f32, z as f32),
                    block,
                    (width, height),
                ));
            }
        }
    }
    world
}

pub fn cube(position: glm::Vec3, tex_coords: (f32, f32), tex_size: (f32, f32)) -> Shape {
    let (width, height) = tex_size;

    let cube_vertices = [
        // front
        (-1.0, -1.0, 1.0),
        (1.0, -1.0, 1.0),
        (1.0, 1.0, 1.0),
        (-1.0, 1.0, 1.0),
        // top
        (-1.0, 1.0, 1.0),
        (1.0, 1.0, 1.0),
        (1.0, 1.0, -1.0),
        (-1.0, 1.0, -1.0),
        // back
        (1.0, -1.0, -1.0),
        (-1.0, -1.0, -1.0),
        (-1.0, 1.0, -1.0),
        (1.0, 1.0, -1.0),
        // bottom
        (-1.0, -1.0, -1.0),
        (1.0, -1.0, -1.0),
        (1.0, -1.0, 1.0),
        (-1.0, -1.0, 1.0),
        // left
        (-1.0, -1.0, -1.0),
        (-1.0, -1.0, 1.0),
        (-1.0, 1.0, 1.0),
        (-1.0, 1.0, -1.0),
        // right
        (1.0, -1.0, 1.0),
        (1.0, -1.0, -1.0),
        (1.0, 1.0, -1.0),
        (1.0, 1.0, 1.0),
    ];

    let cube_texcoords = [
        [(tex_coords.0) / width, (tex_coords.1) / height],
        [(tex_coords.0 + 1.) / width, (tex_coords.1) / height],
        [(tex_coords.0 + 1.) / width, (tex_coords.1 + 1.) / height],
        [(tex_coords.0) / width, (tex_coords.1 + 1.) / height],
    ];
    let mut triangles = Vec::with_capacity(24);

    for vertex in 0..cube_vertices.len() {
        let (x, y, z) = cube_vertices[vertex];
        let x = x / 2. + 0.5 + position.x;
        let y = y / 2. + 0.5 + position.y;
        let z = z / 2. + 0.5 + position.z;
        triangles.push(Vertex::new([x, y, z], cube_texcoords[vertex % 4]));
    }

    let cube_indices = vec![
        0, 1, 2, 2, 3, 0, // front
        4, 5, 6, 6, 7, 4, // top
        8, 9, 10, 10, 11, 8, // back
        12, 13, 14, 14, 15, 12, // bottom
        16, 17, 18, 18, 19, 16, // left
        20, 21, 22, 22, 23, 20, // right
    ];

    Shape::from_vertices(triangles, cube_indices)

    // }
    // Shape::from_vertices(&out_triangle, &(0..out_triangle.len() as u32).collect())
}

pub fn program(display: glium::Display) -> glium::Program {
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = projection * view * model * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap()
}

pub fn draw_param() -> glium::DrawParameters<'static> {
    glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    }
}
