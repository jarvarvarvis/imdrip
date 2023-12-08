pub struct ShaderPart {
    handle: u32,
}

impl ShaderPart {
    pub fn new(kind: gl::types::GLenum) -> Self {
        unsafe {
            let handle = gl::CreateShader(kind);
            Self { handle }
        }
    }

    pub fn handle(&self) -> u32 {
        self.handle
    }

    pub fn set_source(&mut self, source: &str) {
        unsafe {
            use std::ffi::CString;
            let source_ptr = CString::new(source)
                .expect("Source contained internal null byte(s)");

            gl::ShaderSource(
                self.handle,
                1,
                &source_ptr.as_ptr(),
                std::ptr::null()
            );
        }
    }

    pub fn compile_status_code(&mut self) -> i32 {
        unsafe {
            let mut success = 0;
            gl::GetShaderiv(self.handle, gl::COMPILE_STATUS, &mut success);
            success
        }
    }

    pub fn compile_log(&mut self) -> Option<String> {
        if self.compile_status_code() != gl::TRUE as i32 {
            unsafe {
                // Get info log length
                let mut info_log_length = 0;
                gl::GetShaderiv(self.handle, gl::INFO_LOG_LENGTH, &mut info_log_length);

                // Read data
                let mut info_log: Vec<u8> = vec![0; info_log_length as usize];
                gl::GetShaderInfoLog(
                    self.handle, 
                    info_log_length, 
                    std::ptr::null_mut(), 
                    info_log.as_mut_ptr() as *mut i8,
                );

                Some(
                    String::from_utf8(info_log)
                        .expect("Unable to convert shader info log to UTF-8")
                )

            }
        } else {
            None
        }
    }

    pub fn compile(&mut self) -> Result<(), String> {
        unsafe {
            gl::CompileShader(self.handle);

            if let Some(err_log) = self.compile_log() {
                Err(err_log)
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for ShaderPart {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.handle);
        }
    }
}
