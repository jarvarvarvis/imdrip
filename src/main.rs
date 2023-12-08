extern crate glfw;

use glfw::fail_on_errors;
use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(512, 512, "imdrip", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_drag_and_drop_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
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
