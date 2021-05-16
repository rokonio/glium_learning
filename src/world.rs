use glium;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
implement_vertex!(Vertex, position);

impl Vertex {
    pub fn new(position: [f32; 2]) -> Vertex {
        Vertex { position }
    }
}

pub struct Shape {
    pub vertices: Vec<Vertex>,
}

impl Shape {
    pub fn from_vertices(vertices: Vec<Vertex>) -> Shape {
        Shape { vertices }
    }
}

pub fn default_program(display: glium::Display) -> glium::Program {
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
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
