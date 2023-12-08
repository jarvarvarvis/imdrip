extern crate gl;
extern crate glfw;

mod draw;
mod opengl;

use std::path::Path;

use glfw::Context;
use nalgebra::Vector2;

use draw::DrawingCtx;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    let (mut window, events) = glfw
        .create_window(512, 512, "imdrip", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_size_polling(true);
    window.set_key_polling(true);
    window.set_drag_and_drop_polling(true);

    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // Drawing stuff
    let (width, height) = window.get_size();
    let window_size = Vector2::new(width, height);
    let mut drawing_ctx = DrawingCtx::new(window_size);

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw the quad mesh
        drawing_ctx.draw();
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Size(width, height) => unsafe {
                    drawing_ctx.on_window_resize(Vector2::new(width, height));
                    gl::Viewport(0, 0, width, height);
                },
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::FileDrop(paths) => {
                    println!("Dropped file paths:");
                    for path in paths.iter() {
                        println!("{}", path.display());

                        // Read the texture (if it's a file path)
                        let exists_result = Path::try_exists(&path);
                        if let Ok(exists) = exists_result {
                            if exists {
                                drawing_ctx.update_texture(&path);
                            }
                        } else {
                            let err = exists_result.unwrap_err();
                            println!("Failed to check if path exists: {}", err);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
