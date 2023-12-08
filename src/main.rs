extern crate gl;
extern crate glfw;

mod opengl;

use glfw::fail_on_errors;
use glfw::Context;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(512, 512, "imdrip", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_size_polling(true);
    window.set_key_polling(true);
    window.set_drag_and_drop_polling(true);

    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Size(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::FileDrop(paths) => {
                    println!("Dropped file paths:");
                    for path in paths.iter() {
                        println!("{}", path.display());
                    }
                }
                _ => {}
            }
        }
    }
}
