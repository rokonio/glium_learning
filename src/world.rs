#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);

impl Vertex {
    pub fn new(position: [f32; 3]) -> Vertex {
        Vertex { position }
    }
}

pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Shape {
    pub fn from_vertices(vertices: Vec<Vertex>, indices: Vec<u32>) -> Shape {
        Shape { vertices, indices }
    }
}

pub fn default_program(display: glium::Display) -> glium::Program {
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap()
}
