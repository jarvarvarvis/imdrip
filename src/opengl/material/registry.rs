use std::collections::HashMap;
use std::rc::Rc;

use crate::opengl::mesh::Mesh;
use crate::opengl::shader::shader_program::ShaderProgram;
use crate::opengl::texture::texture_2d::Texture2D;

use super::basic::BasicMaterial;
use super::textured::TexturedMaterial;
use super::Material;

pub struct MaterialRegistry {
    material_map: HashMap<String, Rc<dyn Material>>,
}

impl MaterialRegistry {
    pub fn new() -> Self {
        Self {
            material_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, material: Rc<dyn Material>) {
        self.material_map.insert(String::from(name), material);
    }

    pub fn insert_basic(&mut self, name: &str, shader_program: Rc<ShaderProgram>) {
        let material = BasicMaterial::new(shader_program);
        self.insert(name, Rc::new(material));
    }

    pub fn insert_2d_textured(
        &mut self,
        name: &str,
        shader_program: Rc<ShaderProgram>,
        texture: Rc<Texture2D>,
    ) {
        let material = TexturedMaterial::new_single_2d(shader_program, texture);
        self.insert(name, Rc::new(material));
    }

    pub fn get_clone_ref(&self, name: &str) -> Option<Rc<dyn Material>> {
        if let Some(material) = self.material_map.get(name) {
            Some(Rc::clone(&material))
        } else {
            None
        }
    }

    pub fn update_material_for(&self, name: &str, mesh: &mut Mesh) {
        if let Some(material) = self.get_clone_ref(name) {
            mesh.set_material(material);
        }
    }
}
