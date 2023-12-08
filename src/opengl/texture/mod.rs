use std::ops::Deref;

pub mod loading;
pub mod named_texture_bindings;
pub mod texture_2d;

pub fn unbind(target: gl::types::GLenum) {
    unsafe {
        gl::BindTexture(target, 0);
    }
}

pub fn set_active_texture_unit(unit: u32) -> Result<(), String> {
    unsafe {
        if unit >= 16 {
            return Err(format!("Out-of-range texture unit: {}", unit));
        }
        gl::ActiveTexture(gl::TEXTURE0 + unit);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Texture {
    handle: u32,
    target: gl::types::GLenum,
}

impl Texture {
    pub fn new(target: gl::types::GLenum) -> Self {
        unsafe {
            let mut handle = 0;
            gl::GenTextures(1, &mut handle);

            Self { handle, target }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(self.target, self.handle);
        }
    }

    pub fn set_wrap_r(&mut self, wrap_r: gl::types::GLenum) {
        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_R, wrap_r as i32);
        }
    }

    pub fn set_wrap_s(&mut self, wrap_s: gl::types::GLenum) {
        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_S, wrap_s as i32);
        }
    }

    pub fn set_wrap_t(&mut self, wrap_t: gl::types::GLenum) {
        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_T, wrap_t as i32);
        }
    }

    pub fn set_filter_min(&mut self, min_filter: gl::types::GLenum) {
        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, min_filter as i32);
        }
    }

    pub fn set_filter_mag(&mut self, mag_filter: gl::types::GLenum) {
        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
        }
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
        unsafe {
            gl::TexImage2D(
                self.target,
                0,
                storage_format as i32,
                width,
                height,
                0,
                source_format,
                source_data_type,
                data_ptr,
            );
        }
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
        unsafe {
            self.set_image_data_from_raw_ptr(
                image_data.as_ptr() as *const std::ffi::c_void,
                width,
                height,
                storage_format,
                source_format,
                source_data_type,
            );
        }
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
        self.set_image_data_from_slice(
            &image_buffer,
            image_buffer.width() as i32,
            image_buffer.height() as i32,
            storage_format,
            source_format,
            source_data_type,
        );
    }

    pub fn handle(&self) -> u32 {
        self.handle
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            println!("Deleting texture {}", self.handle);
            gl::DeleteTextures(1, &mut self.handle);
        }
    }
}
