extern crate gl;
extern crate glfw;

mod opengl;

use std::path::Path;
use std::rc::Rc;

use glfw::Context;

use opengl::material::textured::*;
use opengl::material::MockMaterial;

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
    let texture_draw_shader = crate::opengl::shader::create_shader_from_parts(
        &include_str!("shaders/quad.vert"),
        &include_str!("shaders/quad.frag"),
    );
    texture_draw_shader.set_int("image_texture", 0);
    let mut texture_draw_material = TexturedMaterial::new(Rc::new(texture_draw_shader), vec![]);

    let quad_mesh =
        crate::opengl::mesh::factory::create_basic_quad_mesh(Rc::new(MockMaterial), 1.0);

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw the quad mesh
        crate::opengl::texture::set_active_texture_unit(0).unwrap();
        quad_mesh.draw_with_material(&texture_draw_material, |_| {});

        window.swap_buffers();
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

                        // Read the texture (if it's a file path)
                        let exists_result = Path::try_exists(&path);
                        if let Ok(exists) = exists_result {
                            if exists {
                                // If no texture is stored yet, create a new one
                                let drawn_textures = texture_draw_material.textures_mut();
                                if drawn_textures.is_empty() {
                                    let load_result =
                                        crate::opengl::texture::loading::create_and_load_texture(
                                            &path,
                                        );

                                    if load_result.is_err() {
                                        println!(
                                            "Failed to load texture: {}",
                                            load_result.unwrap_err()
                                        );
                                        continue;
                                    }

                                    // If the texture was loaded successfully, store it in the
                                    // material used for drawing
                                    let stored_texture = TextureKind::TwoDimensional {
                                        texture: load_result.unwrap(),
                                    };
                                    drawn_textures.push(stored_texture);
                                } else {
                                    // Otherwise, load the texture into the already created texture
                                    let current_texture = &drawn_textures[0];
                                    if let TextureKind::TwoDimensional { texture } =
                                        &current_texture
                                    {
                                        let load_result =
                                            crate::opengl::texture::loading::load_texture_into(
                                                &path, texture,
                                            );

                                        if load_result.is_err() {
                                            println!(
                                                "Failed to load texture: {}",
                                                load_result.unwrap_err()
                                            );
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                println!("File doesn't exist: {}", path.display());
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
