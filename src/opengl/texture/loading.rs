use std::path::Path;
use std::rc::Rc;

use super::texture_2d::Texture2D;

pub fn create_and_load_texture<P: AsRef<Path>>(path: P) -> Rc<Texture2D> {
    let texture = Texture2D::new().setup(|texture| {
        texture.set_wrap_mode(gl::REPEAT, gl::REPEAT);
        texture.set_filter_ops(gl::NEAREST, gl::NEAREST);

        let image = image::open(&path)
            .expect(&format!(
                "Failed to load image: {}",
                path.as_ref().to_string_lossy()
            ))
            .into_rgba8();

        // Image data needs to be flipped vertically!
        let flipped_image = image::imageops::flip_vertical(&image);

        texture.set_image_data(flipped_image, gl::RGBA, gl::RGBA, gl::UNSIGNED_BYTE);
    });
    Rc::new(texture)
}
