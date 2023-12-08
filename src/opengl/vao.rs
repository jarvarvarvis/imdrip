pub struct Vao {
    handle: u32,
}

pub fn unbind() {
    unsafe {
        gl::BindVertexArray(0);
    }
}

impl Vao {
    pub fn new() -> Self {
        unsafe {
            let mut handle = 0;
            gl::GenVertexArrays(1, &mut handle);
            Self { handle }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.handle);
        }
    }

    pub fn setup<SetupFn>(mut self, mut setup: SetupFn) -> Self
    where
        SetupFn: FnMut(&mut Self)
    {
        self.bind();
        setup(&mut self);
        unbind();
        self
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.handle);
        }
    }
}
