use super::buffers;

pub struct Ebo {
    handle: u32,
}

impl Ebo {
    pub fn new() -> Self {
        Self {
            handle: buffers::create_buffer()
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.handle);
        }
    }

    pub fn setup<SetupFn>(mut self, mut setup: SetupFn) -> Self
    where
        SetupFn: FnMut(&mut Self)
    {
        self.bind();
        setup(&mut self);
        buffers::unbind(gl::ELEMENT_ARRAY_BUFFER);
        self
    }

    pub fn copy_data(&mut self, data: &[u32], mode: gl::types::GLenum) {
        buffers::copy_data_to_bound_target(gl::ELEMENT_ARRAY_BUFFER, data, mode);
    }
    
    pub fn copy_data_static(&mut self, data: &[u32]) {
        buffers::copy_data_to_bound_target_static(gl::ELEMENT_ARRAY_BUFFER, data);
    }
}

impl Drop for Ebo {  
    fn drop(&mut self) {
        buffers::delete_buffer(self.handle);
    }
}
