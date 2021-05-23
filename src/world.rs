#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Vertex {
        Vertex {
            position,
            tex_coords,
        }
    }
}

#[derive(Debug)]
pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[allow(unused)]
impl Shape {
    pub fn from_vertices(vertices: Vec<Vertex>, indices: Vec<u32>) -> Shape {
        Shape { vertices, indices }
    }

    pub fn new() -> Shape {
        Shape {
            vertices: vec![],
            indices: vec![],
        }
    }

    pub fn add_vertices(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>) {
        let index_base = self.vertices.len();
        self.vertices.extend(vertices);
        self.indices
            .extend(indices.iter().map(|i| *i + index_base as u32));
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.add_vertices(shape.vertices, shape.indices);
    }

    pub fn indices_and_vertices(
        &self,
        display: &glium::Display,
    ) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u32>) {
        (
            glium::VertexBuffer::new(display, &self.vertices).unwrap(),
            glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &self.indices,
            )
            .unwrap(),
        )
    }
}

pub fn degree_to_radian(degree: f32) -> f32 {
    degree * (3.1415 / 180.)
}
