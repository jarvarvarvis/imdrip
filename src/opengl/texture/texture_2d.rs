use std::ops::Deref;

use super::Texture;

pub fn unbind() {
    super::unbind(gl::TEXTURE_2D);
}

#[derive(Debug)]
pub struct Texture2D {
    texture: Texture,
}

impl Texture2D {
    pub fn new() -> Self {
        Self {
            texture: Texture::new(gl::TEXTURE_2D),
        }
    }

    pub fn bind(&self) {
        self.texture.bind();
    }

    pub fn setup<SetupFn>(mut self, mut setup: SetupFn) -> Self
    where
        SetupFn: FnMut(&mut Self),
    {
        self.bind();
        setup(&mut self);
        unbind();
        self
    }

    pub fn set_wrap_mode(&mut self, wrap_s: gl::types::GLenum, wrap_t: gl::types::GLenum) {
        self.texture.set_wrap_s(wrap_s);
        self.texture.set_wrap_t(wrap_t);
    }

    pub fn set_filter_ops(&mut self, min_filter: gl::types::GLenum, mag_filter: gl::types::GLenum) {
        self.texture.set_filter_min(min_filter);
        self.texture.set_filter_mag(mag_filter);
    }

    pub unsafe fn set_image_data_from_raw_ptr(
        &self,
        data_ptr: *const std::ffi::c_void,
        width: i32,
        height: i32,
        storage_format: gl::types::GLenum,
        source_format: gl::types::GLenum,
        source_data_type: gl::types::GLenum,
    ) {
        self.texture.set_image_data_from_raw_ptr(
            data_ptr,
            width,
            height,
            storage_format,
            source_format,
            source_data_type,
        )
    }

    pub fn set_image_data_from_slice<T>(
        &self,
        image_data: &[T],
        width: i32,
        height: i32,
        storage_format: gl::types::GLenum,
        source_format: gl::types::GLenum,
        source_data_type: gl::types::GLenum,
    ) {
        self.texture.set_image_data_from_slice(
            image_data,
            width,
            height,
            storage_format,
            source_format,
            source_data_type,
        );
    }

    pub fn set_image_data<P, Container>(
        &self,
        image_buffer: image::ImageBuffer<P, Container>,
        storage_format: gl::types::GLenum,
        source_format: gl::types::GLenum,
        source_data_type: gl::types::GLenum,
    ) where
        P: image::Pixel,
        Container: Deref<Target = [P::Subpixel]>,
    {
        self.texture.set_image_data(
            image_buffer,
            storage_format,
            source_format,
            source_data_type,
        );
    }

    pub fn handle(&self) -> u32 {
        self.texture.handle()
    }
}
