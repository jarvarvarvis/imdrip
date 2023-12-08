use super::shader_part::*;

pub struct ShaderProgram {
    handle: u32,
}

pub fn unbind() {
    unsafe {
        gl::UseProgram(0);
    }
}

impl ShaderProgram {
    pub fn new() -> Self {
        unsafe {
            let handle = gl::CreateProgram();
            Self { handle }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    pub fn setup<SetupFn>(mut self, setup: SetupFn) -> Self
    where
        SetupFn: Fn(&mut Self),
    {
        setup(&mut self);
        self
    }

    pub fn link_status(&mut self) -> i32 {
        unsafe {
            let mut success = 0;
            gl::GetProgramiv(self.handle, gl::LINK_STATUS, &mut success);
            success
        }
    }

    pub fn validate_status(&mut self) -> i32 {
        unsafe {
            let mut success = 0;
            gl::GetProgramiv(self.handle, gl::VALIDATE_STATUS, &mut success);
            success
        }
    }

    fn get_info_log(&self) -> String {
        unsafe {
            // Get info log length
            let mut info_log_length = 0;
            gl::GetProgramiv(self.handle, gl::INFO_LOG_LENGTH, &mut info_log_length);

            // Read data
            let mut info_log: Vec<u8> = vec![0; info_log_length as usize];
            gl::GetProgramInfoLog(
                self.handle,
                info_log_length,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8,
            );

            String::from_utf8(info_log).expect("Unable to convert shader info log to UTF-8")
        }
    }

    pub fn link_log(&mut self) -> Option<String> {
        if self.link_status() != gl::TRUE as i32 {
            Some(self.get_info_log())
        } else {
            None
        }
    }

    pub fn validate_log(&mut self) -> Option<String> {
        if self.validate_status() != gl::TRUE as i32 {
            Some(self.get_info_log())
        } else {
            None
        }
    }

    pub fn link_with_parts(&mut self, parts: &[ShaderPart]) -> Result<(), String> {
        unsafe {
            for part in parts.iter() {
                gl::AttachShader(self.handle, part.handle());
            }

            // Link
            gl::LinkProgram(self.handle);
            if let Some(err_log) = self.link_log() {
                return Err(err_log);
            }

            // Validate
            gl::ValidateProgram(self.handle);
            if let Some(err_log) = self.validate_log() {
                return Err(err_log);
            }

            Ok(())
        }
    }

    #[allow(unused)]
    pub fn get_uniform_location(&self, name: &str) -> i32 {
        unsafe {
            use std::ffi::CString;
            let name_cstr =
                CString::new(name).expect("Uniform name contained internal null byte(s)");
            gl::GetUniformLocation(self.handle, name_cstr.as_ptr())
        }
    }

    #[allow(unused)]
    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            let gl_value = if value { gl::TRUE } else { gl::FALSE };
            gl::Uniform1i(self.get_uniform_location(name), gl_value as i32);
        }
    }

    #[allow(unused)]
    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(name), value);
        }
    }

    #[allow(unused)]
    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(name), value);
        }
    }

    #[allow(unused)]
    pub fn set_vec2f(&self, name: &str, value: nalgebra::base::Vector2<f32>) {
        unsafe {
            gl::Uniform2f(self.get_uniform_location(name), value.x, value.y);
        }
    }

    #[allow(unused)]
    pub fn set_vec2i(&self, name: &str, value: nalgebra::base::Vector2<i32>) {
        unsafe {
            gl::Uniform2i(self.get_uniform_location(name), value.x, value.y);
        }
    }

    #[allow(unused)]
    pub fn set_vec3f(&self, name: &str, value: nalgebra::base::Vector3<f32>) {
        unsafe {
            gl::Uniform3f(self.get_uniform_location(name), value.x, value.y, value.z);
        }
    }

    #[allow(unused)]
    pub fn set_vec3f_array(&self, name: &str, values: &[nalgebra::base::Vector3<f32>]) {
        unsafe {
            let ptr = values.as_ptr() as *const f32;
            gl::Uniform3fv(self.get_uniform_location(name), values.len() as i32, ptr);
        }
    }

    #[allow(unused)]
    pub fn set_mat3f(&self, name: &str, value: nalgebra::base::Matrix3<f32>) {
        unsafe {
            gl::UniformMatrix3fv(
                self.get_uniform_location(name),
                1,
                gl::FALSE,
                value.as_ptr(),
            );
        }
    }

    #[allow(unused)]
    pub fn set_mat4f(&self, name: &str, value: nalgebra::base::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.get_uniform_location(name),
                1,
                gl::FALSE,
                value.as_ptr(),
            )
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}
