use std::rc::Rc;

use crate::opengl::shader::shader_program::ShaderProgram;

use super::Material;

pub struct BasicMaterial {
    shader_program: Rc<ShaderProgram>,
}

impl BasicMaterial {
    pub fn new(shader_program: Rc<ShaderProgram>) -> Self {
        Self { shader_program }
    }
}

impl Material for BasicMaterial {
    fn bind(&self) {
        self.shader_program.bind();
    }

    fn unbind(&self) {
        crate::opengl::shader::shader_program::unbind();
    }

    fn shader_program(&self) -> &ShaderProgram {
        &self.shader_program
    }
}
