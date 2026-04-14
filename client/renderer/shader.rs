pub struct Shader {
    pub vertex_source: String,
    pub fragment_source: String,
}

impl Shader {
    pub fn new(vertex: &str, fragment: &str) -> Self {
        Self {
            vertex_source: vertex.into(),
            fragment_source: fragment.into(),
        }
    }

    pub fn default() -> Self {
        Self {
            vertex_source: "
                #version 450
                layout(location = 0) in vec3 a_pos;
                void main() {
                    gl_Position = vec4(a_pos, 1.0);
                }
            ".into(),
            fragment_source: "
                #version 450
                layout(location = 0) out vec4 color;
                void main() {
                    color = vec4(0.3, 0.7, 1.0, 1.0);
                }
            ".into(),
        }
    }
}
