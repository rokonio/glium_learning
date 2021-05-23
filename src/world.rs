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

pub fn default_program(display: glium::Display) -> glium::Program {
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

pub fn default_draw_param() -> glium::DrawParameters<'static> {
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

pub fn degree_to_radian(degree: f32) -> f32 {
    degree * (3.1415 / 180.)
}
