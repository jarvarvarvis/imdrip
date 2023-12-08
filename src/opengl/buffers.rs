pub fn unbind(target: gl::types::GLenum) {
    unsafe {
        gl::BindBuffer(target, 0);
    }
}


pub fn copy_data_to_bound_target<T>(
    target: gl::types::GLenum, 
    data: &[T], 
    mode: gl::types::GLenum
) {
    unsafe {
        use std::mem::size_of;
        gl::BufferData(
            target,
            (size_of::<T>() * data.len()) as isize,
            data.as_ptr() as *mut std::ffi::c_void,
            mode,
        )
    }
}

pub fn copy_data_to_bound_target_static<T>(target: gl::types::GLenum, data: &[T]) {
    copy_data_to_bound_target(target, data, gl::STATIC_DRAW);
}

pub fn create_buffer() -> u32 {
    unsafe {
        let mut handle = 0;
        gl::GenBuffers(1, &mut handle);
        handle
    }
}

pub fn delete_buffer(handle: u32) {
    unsafe {
        gl::DeleteBuffers(1, &handle);
    }
}
