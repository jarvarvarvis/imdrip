use std::path::Path;
use std::rc::Rc;

use image::RgbaImage;
use nalgebra::Vector2;

use super::texture_2d::Texture2D;

pub fn create_from_image(image: RgbaImage) -> (Rc<Texture2D>, Vector2<i32>) {
    let mut texture = Texture2D::new();
    texture.bind();

    let mut size;
    {
        texture.set_wrap_mode(gl::REPEAT, gl::REPEAT);
        texture.set_filter_ops(gl::NEAREST, gl::NEAREST);

        // Image data needs to be flipped vertically!
        let (width, height) = image.dimensions();
        size = Vector2::new(width as i32, height as i32);

        texture.set_image_data(image, gl::RGBA, gl::RGBA, gl::UNSIGNED_BYTE);
    }

    crate::opengl::texture::texture_2d::unbind();
    (Rc::new(texture), size)
}

pub fn create_and_load_texture_from_path<P: AsRef<Path>>(
    path: P,
) -> Result<(Rc<Texture2D>, Vector2<i32>), String> {
    let image = image::open(&path)
        .map_err(|_| format!("Failed to load image: {}", path.as_ref().to_string_lossy()))?
        .into_rgba8();

    let flipped_image = image::imageops::flip_vertical(&image);
    Ok(create_from_image(flipped_image))
}

pub fn load_from_image_into_texture(image: RgbaImage, texture: &Texture2D) -> Vector2<i32> {
    let (width, height) = image.dimensions();

    texture.bind();
    texture.set_image_data(image, gl::RGBA, gl::RGBA, gl::UNSIGNED_BYTE);
    crate::opengl::texture::texture_2d::unbind();

    let size = Vector2::new(width as i32, height as i32);
    size
}

pub fn load_into_texture_from_path<P: AsRef<Path>>(
    path: P,
    texture: &Texture2D,
) -> Result<Vector2<i32>, String> {
    let image = image::open(&path)
        .map_err(|_| format!("Failed to load image: {}", path.as_ref().to_string_lossy()))?
        .into_rgba8();

    let flipped_image = image::imageops::flip_vertical(&image);
    Ok(load_from_image_into_texture(flipped_image, texture))
}
