use std::mem::size_of;

use super::buffers;

pub struct Vbo {
    handle: u32,
}

impl Vbo {
    pub fn new() -> Self {
        Self {
            handle: buffers::create_buffer()
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.handle);
        }
    }

    pub fn setup<SetupFn>(mut self, mut setup: SetupFn) -> Self
    where
        SetupFn: FnMut(&mut Self)
    {
        self.bind();
        setup(&mut self);
        buffers::unbind(gl::ARRAY_BUFFER);
        self
    }

    pub fn copy_data<T>(&mut self, data: &[T], mode: gl::types::GLenum) {
        buffers::copy_data_to_bound_target(gl::ARRAY_BUFFER, data, mode);
    }

    pub fn copy_data_static<T>(&mut self, data: &[T]) {
        buffers::copy_data_to_bound_target_static(gl::ARRAY_BUFFER, data);
    }

    pub unsafe fn set_vertex_attrib_pointer(
        &mut self,
        location: u32,
        size: i32,
        data_type: gl::types::GLenum,
        normalized: bool,
        stride: i32,
        offset_pointer: *const std::ffi::c_void,
    ) {
        gl::VertexAttribPointer(
            location,
            size,
            data_type,
            if normalized { gl::TRUE } else { gl::FALSE },
            stride,
            offset_pointer,
        );
    }

    pub fn set_basic_typed_vertex_attrib_pointer<T>(
        &mut self,
        location: u32,
        size: i32,
        data_type: gl::types::GLenum,
        normalized: bool,
    ) {
        unsafe {
            self.set_vertex_attrib_pointer(
                location,
                size,
                data_type,
                normalized,
                (size as usize * size_of::<T>()) as i32,
                std::ptr::null(),
            )
        }
    }

    pub fn set_vertex_attrib_enabled(&mut self, location: u32, state: bool) {
        unsafe {
            if state {
                gl::EnableVertexAttribArray(location);
            } else {
                gl::DisableVertexAttribArray(location);
            }
        }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        buffers::delete_buffer(self.handle);
    }
}
