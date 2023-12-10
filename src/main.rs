extern crate gl;
extern crate glfw;

mod imdrip;
mod opengl;

use glfw::Context;
use nalgebra::Vector2;

use imdrip::ImdripCtx;

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
    let mut drawing_ctx = ImdripCtx::new(window_size);

    // Handle command line args
    let mut args = std::env::args();
    if args.len() == 2 {
        let executable_name = args.next().unwrap();
        let argument = args.next();

        if let Some(first_argument) = argument {
            if first_argument == "help" {
                println!("Usage:");
                println!("{} help            - Show this help", executable_name);
                println!(
                    "{} <file path/url> - Load an image from disk/url",
                    executable_name
                );
                return;
            }

            let path = std::path::PathBuf::from(&first_argument);
            let successfully_loaded = drawing_ctx.handle_file_path(&path);

            // Resize (if resize-on-load is enabled)
            if successfully_loaded && drawing_ctx.resize_on_load() {
                let size = drawing_ctx.image_size();
                window.set_size(size.x, size.y);
            }
        }
    }

    // Main loop
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
                glfw::WindowEvent::Key(glfw::Key::R, _, glfw::Action::Press, _) => {
                    drawing_ctx.toggle_resize_on_load();
                    let status = if drawing_ctx.resize_on_load() {
                        "Enabled"
                    } else {
                        "Disabled"
                    };
                    println!("{} resize on load", status);
                }
                glfw::WindowEvent::Key(glfw::Key::F, _, glfw::Action::Press, _) => {
                    let size = drawing_ctx.image_size();
                    if size.x == 0 || size.y == 0 {
                        println!("Image has width or height of zero (which might break resizing)!");
                        continue;
                    }

                    window.set_size(size.x, size.y);
                    println!("Resized window to {}, {} to fit image", size.x, size.y);
                }
                glfw::WindowEvent::FileDrop(paths) => {
                    for path in paths.iter() {
                        let successfully_loaded = drawing_ctx.handle_file_path(path);

                        // Resize (if resize-on-load is enabled)
                        if successfully_loaded && drawing_ctx.resize_on_load() {
                            let size = drawing_ctx.image_size();
                            window.set_size(size.x, size.y);
                        }
                    }
                }
                _ => {}
            }
        }

        // Set window title
        let resize_on_load_status = if drawing_ctx.resize_on_load() {
            "Enabled"
        } else {
            "Disabled"
        };

        window.set_title(&format!("imdrip - Resize on load? {}", resize_on_load_status));
    }
}
