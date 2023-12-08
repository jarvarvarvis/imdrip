use std::rc::Rc;

use super::ebo::Ebo;
use super::material::Material;
use super::vao::Vao;
use super::vbo::Vbo;

pub mod builder;
pub mod factory;

pub enum DrawMode {
    Arrays {
        array_mode: gl::types::GLenum,
        vertices: i32,
    },
    Elements {
        ebo: Ebo,
        elements_mode: gl::types::GLenum,
        vertices: i32,
    },
}

pub struct Mesh {
    vao: Vao,

    #[allow(unused)]
    vbos: Vec<Vbo>,

    // Materials are not safely clonable/copyable, since they contain Shaders and
    // Textures, which hold handles managed by OpenGL.
    //
    // Cloning/copying either a Shader or Texture object can result in scenarios
    // where an object (that was cloned/copied) is dropped in two places,
    // therefore calling gl::DeleteProgram/gl::DeleteTextures twice for the same
    // object handle.
    //
    // Only a reference-counted pointer to a Material is stored for each mesh,
    // so that it's possible to reuse once-created Materials for multiple meshes.
    material: Rc<dyn Material>,

    draw_mode: DrawMode,
}

impl Mesh {
    pub fn new(vao: Vao, vbos: Vec<Vbo>, material: Rc<dyn Material>, draw_mode: DrawMode) -> Self {
        Self {
            vao,
            vbos,
            material,
            draw_mode,
        }
    }

    pub fn draw_with_material<PreDrawOp>(&self, material: &dyn Material, mut pre_draw_op: PreDrawOp)
    where
        PreDrawOp: FnMut(&Self),
    {
        material.bind();
        self.vao.bind();

        pre_draw_op(self);

        // Draw
        match &self.draw_mode {
            DrawMode::Arrays {
                array_mode,
                vertices,
            } => unsafe {
                gl::DrawArrays(*array_mode, 0, *vertices);
            },
            DrawMode::Elements {
                ebo,
                elements_mode,
                vertices,
            } => unsafe {
                ebo.bind();
                gl::DrawElements(
                    *elements_mode,
                    *vertices,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                super::buffers::unbind(gl::ELEMENT_ARRAY_BUFFER);
            },
        }

        super::vao::unbind();

        material.unbind();
    }

    pub fn draw<PreDrawOp>(&self, pre_draw_op: PreDrawOp)
    where
        PreDrawOp: FnMut(&Self),
    {
        self.draw_with_material(self.material.as_ref(), pre_draw_op);
    }

    pub fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }

    pub fn set_material(&mut self, material: Rc<dyn Material>) {
        self.material = material;
    }
}
