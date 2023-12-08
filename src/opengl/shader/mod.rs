pub mod shader_part;
pub mod shader_program;

use shader_part::*;
use shader_program::*;

pub fn create_shader_from_parts(vert_source: &str, frag_source: &str) -> ShaderProgram {
    let program = ShaderProgram::new().setup(|program| {
        let mut vert_shader = ShaderPart::new(gl::VERTEX_SHADER);
        vert_shader.set_source(vert_source);
        vert_shader
            .compile()
            .expect("Vertex shader compilation failed");

        let mut frag_shader = ShaderPart::new(gl::FRAGMENT_SHADER);
        frag_shader.set_source(frag_source);
        frag_shader
            .compile()
            .expect("Fragment shader compilation failed");

        program
            .link_with_parts(&[vert_shader, frag_shader])
            .expect("Program linking failed");
    });
    program
}
