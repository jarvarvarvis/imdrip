use crate::opengl::shader::shader_program::ShaderProgram;

pub mod basic;
pub mod registry;
pub mod textured;

pub trait Material {
    fn bind(&self);
    fn unbind(&self);

    fn shader_program(&self) -> &ShaderProgram;
}

pub struct MockMaterial;

impl Material for MockMaterial {
    fn bind(&self) {
        unimplemented!()
    }

    fn unbind(&self) {
        unimplemented!()
    }

    fn shader_program(&self) -> &ShaderProgram {
        panic!("Mock material doesn't store shader")
    }
}
