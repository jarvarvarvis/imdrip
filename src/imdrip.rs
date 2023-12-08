use std::path::{Path, PathBuf};
use std::rc::Rc;

use image::RgbaImage;
use nalgebra::Vector2;

use crate::opengl::material::textured::{TextureKind, TexturedMaterial};
use crate::opengl::material::{Material, MockMaterial};
use crate::opengl::mesh::Mesh;
use crate::opengl::texture::texture_2d::Texture2D;

pub struct ImdripCtx {
    material: TexturedMaterial,
    current_image_size: Vector2<i32>,
    current_window_size: Vector2<i32>,
    resize_on_load: bool,
    mesh: Mesh,
}

impl ImdripCtx {
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
            current_image_size: Vector2::new(0, 0),
            current_window_size,
            resize_on_load: true,
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

    pub fn on_window_resize(&mut self, window_size: Vector2<i32>) {
        self.current_window_size = window_size;
    }

    fn update_existing_texture_from_path<P: AsRef<Path>>(&mut self, path: P) {
        if let Some(tex) = self.get_texture() {
            let load_result =
                crate::opengl::texture::loading::load_into_texture_from_path(&path, tex.as_ref());

            if load_result.is_err() {
                println!("Failed to load texture: {}", load_result.unwrap_err());
                return;
            }

            let size = load_result.unwrap();
            self.current_image_size = size;
        }
    }

    fn load_new_texture_from_path<P: AsRef<Path>>(&mut self, path: P) {
        let load_result = crate::opengl::texture::loading::create_and_load_texture_from_path(&path);
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

        self.current_image_size = size;
    }

    pub fn update_texture_from_path<P: AsRef<Path>>(&mut self, path: P) {
        if self.has_textures() {
            self.update_existing_texture_from_path(path);
            return;
        }

        self.load_new_texture_from_path(path);
    }

    fn update_existing_texture_from_image(&mut self, image: RgbaImage) {
        if let Some(tex) = self.get_texture() {
            let size =
                crate::opengl::texture::loading::load_from_image_into_texture(image, tex.as_ref());

            self.current_image_size = size;
        }
    }

    fn load_new_texture_from_image(&mut self, image: RgbaImage) {
        let image = crate::opengl::texture::loading::create_from_image(image);

        // If the texture was loaded successfully, store it in the
        // material used for drawing
        let (texture, size) = image;
        let stored_texture = TextureKind::TwoDimensional { texture };

        let textures = self.material.textures_mut();
        textures.push(stored_texture);

        self.current_image_size = size;
    }

    pub fn update_texture_from_image(&mut self, image: RgbaImage) {
        if self.has_textures() {
            self.update_existing_texture_from_image(image);
            return;
        }

        self.load_new_texture_from_image(image);
    }

    pub fn handle_file_path(&mut self, path: &PathBuf) -> bool {
        // Read the texture (if it's a file path)
        let exists_result = Path::try_exists(&path);
        let exists = exists_result.map(|exists| exists).unwrap_or(false);
        if exists {
            self.update_texture_from_path(&path);
            return true;
        } else {
            println!("File doesn't exist, trying to download from the internet");
        }

        // Download the texture from a URL (if it is one) and update the drawing ctx
        let downloaded_image = reqwest::blocking::get(path.to_string_lossy().as_ref());
        if let Err(error) = downloaded_image {
            println!("Failed to download image from URL: {}", error);
            return false;
        }

        let response = downloaded_image.unwrap();
        let bytes = response.bytes();
        if let Err(error) = bytes {
            println!("Failed to get full response body as bytes: {}", error);
            return false;
        }

        let received_bytes = bytes.unwrap();
        println!("Received {} bytes", received_bytes.len());

        let image = image::load_from_memory(&received_bytes);
        if let Err(error) = image {
            println!("Failed to create image from response: {}", error);
            return false;
        }

        println!("Done loading image from URL!");
        let image = image.unwrap();
        let flipped_image = image::imageops::flip_vertical(&image);
        self.update_texture_from_image(flipped_image);

        true
    }

    pub fn image_size(&self) -> Vector2<i32> {
        self.current_image_size
    }

    pub fn set_resize_on_load(&mut self, resize_on_load: bool) {
        self.resize_on_load = resize_on_load;
    }

    pub fn toggle_resize_on_load(&mut self) {
        self.set_resize_on_load(!self.resize_on_load);
    }

    pub fn resize_on_load(&self) -> bool {
        self.resize_on_load
    }
}
