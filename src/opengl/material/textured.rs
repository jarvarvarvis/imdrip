use std::rc::Rc;

use crate::opengl::shader::shader_program::ShaderProgram;
use crate::opengl::texture::texture_2d::Texture2D;

use super::Material;

pub enum TextureKind {
    TwoDimensional { texture: Rc<Texture2D> },
}

impl TextureKind {
    pub fn bind(&self) {
        match self {
            TextureKind::TwoDimensional { texture } => texture.bind(),
        }
    }

    pub fn unbind(&self) {
        match self {
            TextureKind::TwoDimensional { texture: _ } => {
                crate::opengl::texture::texture_2d::unbind()
            }
        }
    }
}

pub struct TexturedMaterial {
    shader_program: Rc<ShaderProgram>,
    textures: Vec<TextureKind>,
}

impl TexturedMaterial {
    pub fn new(shader_program: Rc<ShaderProgram>, textures: Vec<TextureKind>) -> Self {
        Self {
            shader_program,
            textures,
        }
    }

    pub fn new_single_2d(shader_program: Rc<ShaderProgram>, texture: Rc<Texture2D>) -> Self {
        Self::new(
            shader_program,
            vec![TextureKind::TwoDimensional { texture }],
        )
    }
}

impl Material for TexturedMaterial {
    fn bind(&self) {
        self.shader_program.bind();

        // Bind textures
        for (unit, texture) in self.textures.iter().enumerate() {
            crate::opengl::texture::set_active_texture_unit(unit as u32).unwrap();
            texture.bind();
        }
    }

    fn unbind(&self) {
        // Unbind textures
        for (unit, texture) in self.textures.iter().enumerate() {
            crate::opengl::texture::set_active_texture_unit(unit as u32).unwrap();
            texture.unbind();
        }

        crate::opengl::shader::shader_program::unbind();
    }

    fn shader_program(&self) -> &ShaderProgram {
        &self.shader_program
    }
}
