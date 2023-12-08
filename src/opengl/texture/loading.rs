use std::path::Path;
use std::rc::Rc;

use super::texture_2d::Texture2D;

pub fn create_and_load_texture<P: AsRef<Path>>(path: P) -> Result<Rc<Texture2D>, String> {
    let mut texture = Texture2D::new();
    texture.bind();

    {
        texture.set_wrap_mode(gl::REPEAT, gl::REPEAT);
        texture.set_filter_ops(gl::NEAREST, gl::NEAREST);

        let image = image::open(&path)
            .map_err(|_| format!("Failed to load image: {}", path.as_ref().to_string_lossy()))?
            .into_rgba8();

        // Image data needs to be flipped vertically!
        let flipped_image = image::imageops::flip_vertical(&image);

        texture.set_image_data(flipped_image, gl::RGBA, gl::RGBA, gl::UNSIGNED_BYTE);
    }

    crate::opengl::texture::texture_2d::unbind();
    Ok(Rc::new(texture))
}

pub fn load_texture_into<P: AsRef<Path>>(path: P, texture: &Texture2D) -> Result<(), String> {
    let image = image::open(&path)
        .map_err(|_| format!("Failed to load image: {}", path.as_ref().to_string_lossy()))?
        .into_rgba8();

    let flipped_image = image::imageops::flip_vertical(&image);
    texture.bind();
    texture.set_image_data(flipped_image, gl::RGBA, gl::RGBA, gl::UNSIGNED_BYTE);
    crate::opengl::texture::texture_2d::unbind();

    Ok(())
}
