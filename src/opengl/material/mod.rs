use crate::opengl::shader::shader_program::ShaderProgram;

pub mod registry;
pub mod basic;
pub mod textured;

pub trait Material {
    fn bind(&self);
    fn unbind(&self);

    fn shader_program(&self) -> &ShaderProgram;
}
