#[derive(Copy, Clone)]
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
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 1.0);
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
