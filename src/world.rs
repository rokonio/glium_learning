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
    vertices: Option<Vec<Vertex>>,
    indices: Option<Vec<u32>>,
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

    pub fn empty() -> Shape {
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
            vertices: None,
            indices: None,
        }
    }
    pub fn from_camera(camera: Camera) -> World {
        World {
            shapes: vec![],
            camera,
            vertices: None,
            indices: None,
        }
    }

    pub fn vertices_and_indices(
        &mut self,
        display: &glium::Display,
    ) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u32>) {
        let mut vertices: Vec<_>;
        let mut indices: Vec<_>;

        if let (Some(v), Some(i)) = (self.vertices.as_ref(), self.indices.as_ref()) {
            return (
                glium::VertexBuffer::new(display, &v).unwrap(),
                glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &i)
                    .unwrap(),
            );
        } else {
            vertices = Vec::with_capacity(
                self.shapes
                    .iter()
                    .map(|shape| shape.vertices.len())
                    .sum::<usize>(),
            );
            indices = Vec::with_capacity(
                self.shapes
                    .iter()
                    .map(|shape| shape.indices.len())
                    .sum::<usize>(),
            );

            for shape in &self.shapes {
                indices.extend(
                    shape
                        .indices
                        .iter()
                        .map(|index| index + vertices.len() as u32),
                );
                vertices.extend(shape.vertices.clone());
            }
            self.vertices = Some(vertices.to_vec());
            self.indices = Some(indices.to_vec());
        }

        (
            glium::VertexBuffer::new(display, &vertices).unwrap(),
            glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &indices,
            )
            .unwrap(),
        )
    }

    pub fn add_shape(&mut self, shape: Shape) {
        if let (Some(ref mut vertices), Some(ref mut indices)) =
            (&mut self.vertices, &mut self.indices)
        {
            let index_base = vertices.len();
            vertices.extend(shape.vertices.clone());
            indices.extend(shape.indices.iter().map(|index| *index + index_base as u32));
        }
        self.shapes.push(shape);
    }
}

pub fn degree_to_radian(degree: f32) -> f32 {
    degree * (std::f32::consts::PI / 180.)
}
