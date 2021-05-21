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

pub fn shapes() -> Shape {
    let width = 24f32;
    let height = 42f32;

    let block1 = (1f32, 41f32); // Stone
    let block2 = (0f32, 40f32); // Cobble stone

    let mut shape = Shape::from_vertices(
        vec![
            Vertex::new([-1.0, -0.5, -1.0], [block1.0 / width, block1.1 / height]),
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
        vec![
            0, 1, 2, // Indices
            0, 2, 3,
        ],
    );

    shape.add_vertices(
        vec![
            Vertex::new([0.0, -0.5, 0.0], [block2.0 / width, block2.1 / height]),
            Vertex::new(
                [0.0, 0.5, 0.0],
                [block2.0 / width, (block2.1 + 1.) / height],
            ),
            Vertex::new(
                [0.5, 0.5, 0.0],
                [(block2.0 + 1.) / width, (block2.1 + 1.) / height],
            ),
            Vertex::new(
                [0.5, -0.5, 0.0],
                [(block2.0 + 1.) / width, block2.1 / height],
            ),
        ],
        vec![
            0, 1, 2, // Indices
            0, 2, 3,
        ],
    );

    shape.add_vertices(
        vec![
            Vertex::new([0.0, 0.5, 0.0], [block2.0 / width, block2.1 / height]),
            Vertex::new(
                [0.0, 0.5, -1.],
                [block2.0 / width, (block2.1 + 1.) / height],
            ),
            Vertex::new(
                [0.5, 0.5, -1.],
                [(block2.0 + 1.) / width, (block2.1 + 1.) / height],
            ),
            Vertex::new(
                [0.5, 0.5, 0.0],
                [(block2.0 + 1.) / width, block2.1 / height],
            ),
        ],
        vec![
            0, 1, 2, // Indices
            0, 2, 3,
        ],
    );
    shape
}
