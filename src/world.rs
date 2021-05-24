use super::Camera;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

#[derive(Debug)]
pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Debug)]
pub struct World {
    pub shapes: Vec<Shape>,
    pub camera: Camera,
}

implement_vertex!(Vertex, position, tex_coords);

#[allow(unused)]
impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Vertex {
        Vertex {
            position,
            tex_coords,
        }
    }
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
}

#[allow(unused)]
impl World {
    pub fn empty() -> World {
        World {
            shapes: vec![],
            camera: Camera::new(0.2, 0.4, glm::vec3(0., 0., 0.), 0., 0.),
        }
    }
    pub fn from_camera(camera: Camera) -> World {
        World {
            shapes: vec![],
            camera,
        }
    }

    pub fn vertices_and_indices(
        &self,
        display: &glium::Display,
    ) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index_indice = 0;
        for shape in &self.shapes {
            vertices.extend(shape.vertices.clone());
            indices.extend(shape.indices.iter().map(|i| *i + index_indice as u32));
            index_indice += shape.vertices.len();
        }

        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();
        let vertices = glium::VertexBuffer::new(display, &vertices).unwrap();

        (vertices, indices)
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
}

pub fn degree_to_radian(degree: f32) -> f32 {
    degree * (3.1415 / 180.)
}
