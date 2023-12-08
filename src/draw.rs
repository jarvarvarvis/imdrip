use std::path::Path;
use std::rc::Rc;

use crate::opengl::material::textured::{TextureKind, TexturedMaterial};
use crate::opengl::material::MockMaterial;
use crate::opengl::mesh::Mesh;
use crate::opengl::texture::texture_2d::Texture2D;

pub struct DrawingCtx {
    material: TexturedMaterial,
    mesh: Mesh,
}

impl DrawingCtx {
    pub fn new() -> Self {
        let texture_draw_shader = crate::opengl::shader::create_shader_from_parts(
            &include_str!("shaders/quad.vert"),
            &include_str!("shaders/quad.frag"),
        );
        texture_draw_shader.set_int("image_texture", 0);
        let material = TexturedMaterial::new(Rc::new(texture_draw_shader), vec![]);

        let mesh = crate::opengl::mesh::factory::create_basic_quad_mesh(Rc::new(MockMaterial), 1.0);

        Self { material, mesh }
    }

    pub fn draw(&self) {
        crate::opengl::texture::set_active_texture_unit(0).unwrap();
        self.mesh.draw_with_material(&self.material, |_| {});
    }

    fn has_textures(&self) -> bool {
        let textures = self.material.textures();
        textures.len() > 0
    }

    fn get_texture(&self) -> Option<&Rc<Texture2D>> {
        let textures = self.material.textures();
        if let TextureKind::TwoDimensional { texture } = textures.get(0)? {
            Some(texture)
        } else {
            None
        }
    }

    fn load_new_texture<P: AsRef<Path>>(&mut self, path: P) {
        if let Some(tex) = self.get_texture() {
            let load_result =
                crate::opengl::texture::loading::load_texture_into(&path, tex.as_ref());

            if load_result.is_err() {
                println!("Failed to load texture: {}", load_result.unwrap_err());
            }
        }
    }

    fn update_existing_texture<P: AsRef<Path>>(&mut self, path: P) {
        let load_result = crate::opengl::texture::loading::create_and_load_texture(&path);
        if load_result.is_err() {
            println!("Failed to load texture: {}", load_result.unwrap_err());
            return;
        }

        // If the texture was loaded successfully, store it in the
        // material used for drawing
        let stored_texture = TextureKind::TwoDimensional {
            texture: load_result.unwrap(),
        };

        let textures = self.material.textures_mut();
        textures.push(stored_texture);
    }

    pub fn update_texture<P: AsRef<Path>>(&mut self, path: P) {
        if self.has_textures() {
            self.load_new_texture(path);
            return;
        }

        self.update_existing_texture(path);
    }
}
