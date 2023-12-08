use std::rc::Rc;

use crate::opengl::ebo::Ebo;
use crate::opengl::material::Material;
use crate::opengl::vao::Vao;
use crate::opengl::vbo::Vbo;

use super::{Mesh, DrawMode};

#[derive(Default)]
pub struct MeshBuilder {
    material: Option<Rc<dyn Material>>,
    vao: Option<Vao>,
    vbos: Vec<Vbo>,

    primitive_mode: Option<gl::types::GLenum>,
    vertex_count: Option<i32>,

    ebo: Option<Ebo>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self {
            vao: Some(Vao::new()),
            ..Default::default()
        }
    }

    pub fn set_material(mut self, material: Rc<dyn Material>) -> Self {
        self.material = Some(material);
        self
    }

    pub fn add_vbo<VboSetupFn>(mut self, setup: VboSetupFn) -> Self
    where
        VboSetupFn: FnMut(&mut Vbo)
    {
        let vao_ref = self.vao.as_ref().unwrap();

        vao_ref.bind();
        let vbo = Vbo::new().setup(setup);
        crate::opengl::vao::unbind();

        self.vbos.push(vbo);
        self
    }

    pub fn set_vertex_count(mut self, vertex_count: i32) -> Self {
        self.vertex_count = Some(vertex_count);
        self
    }

    pub fn set_primitive_mode(mut self, primitive_mode: gl::types::GLenum) -> Self {
        self.primitive_mode = Some(primitive_mode);
        self
    }

    pub fn set_indices(mut self, indices: &[u32]) -> Self {
        let ebo = Ebo::new().setup(|ebo| {
            ebo.copy_data_static(&indices);
        });
        self.ebo = Some(ebo);
        self.vertex_count = Some(indices.len() as i32);
        self
    }

    pub fn build(self) -> Mesh {
        let primitive_mode = self.primitive_mode.expect("Draw mode for primitives was not set");

        let draw_mode = match self.ebo {
            Some(ebo) => DrawMode::Elements {
                ebo,
                elements_mode: primitive_mode,
                vertices: self.vertex_count.unwrap(),
            },
            None => DrawMode::Arrays {
                array_mode: primitive_mode,
                vertices: self.vertex_count.expect("Vertex count was not set"),
            }
        };
        let mesh = Mesh::new(
            self.vao.unwrap(),
            self.vbos,
            self.material.expect("Material was not set"),
            draw_mode,
        );
        mesh
    }
}
