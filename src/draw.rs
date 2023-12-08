use std::path::Path;
use std::rc::Rc;

use nalgebra::Vector2;

use crate::opengl::material::textured::{TextureKind, TexturedMaterial};
use crate::opengl::material::{Material, MockMaterial};
use crate::opengl::mesh::Mesh;
use crate::opengl::texture::texture_2d::Texture2D;

pub struct DrawingCtx {
    material: TexturedMaterial,
    current_texture_size: Vector2<i32>,
    current_window_size: Vector2<i32>,
    mesh: Mesh,
}

impl DrawingCtx {
    pub fn new(current_window_size: Vector2<i32>) -> Self {
        let texture_draw_shader = crate::opengl::shader::create_shader_from_parts(
            &include_str!("shaders/quad.vert"),
            &include_str!("shaders/quad.frag"),
        );
        texture_draw_shader.set_int("image_texture", 0);
        let material = TexturedMaterial::new(Rc::new(texture_draw_shader), vec![]);

        let mesh = crate::opengl::mesh::factory::create_basic_quad_mesh(Rc::new(MockMaterial), 1.0);

        Self {
            material,
            current_texture_size: Vector2::new(0, 0),
            current_window_size,
            mesh,
        }
    }

    pub fn draw(&self) {
        crate::opengl::texture::set_active_texture_unit(0).unwrap();
        self.mesh.draw_with_material(&self.material, |_| {
            let shader = self.material.shader_program();
            shader.set_vec2i("window_size", self.current_window_size);
        });
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
                return;
            }

            let size = load_result.unwrap();
            self.current_texture_size = size;
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
        let (texture, size) = load_result.unwrap();
        let stored_texture = TextureKind::TwoDimensional { texture };

        let textures = self.material.textures_mut();
        textures.push(stored_texture);

        self.current_texture_size = size;
    }

    pub fn on_window_resize(&mut self, window_size: Vector2<i32>) {
        self.current_window_size = window_size;
    }

    pub fn update_texture<P: AsRef<Path>>(&mut self, path: P) {
        if self.has_textures() {
            self.load_new_texture(path);
            return;
        }

        self.update_existing_texture(path);
    }
}
