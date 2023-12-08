use nalgebra::{Vector2, Vector3};
use std::rc::Rc;

use super::builder::MeshBuilder;
use super::Mesh;

use crate::opengl::material::Material;

pub fn create_basic_quad_mesh(material: Rc<dyn Material>, tex_scale: f32) -> Mesh {
    MeshBuilder::new()
        // Position
        .add_vbo(|vbo| {
            vbo.copy_data_static::<Vector3<f32>>(&[
                Vector3::new(-1.0, 0.0, 1.0),
                Vector3::new(1.0, 0.0, 1.0),
                Vector3::new(1.0, 0.0, -1.0),
                Vector3::new(-1.0, 0.0, -1.0),
            ]);
            vbo.set_basic_typed_vertex_attrib_pointer::<f32>(0, 3, gl::FLOAT, false);
            vbo.set_vertex_attrib_enabled(0, true);
        })
        // Normal
        .add_vbo(|vbo| {
            vbo.copy_data_static::<Vector3<f32>>(&[
                Vector3::y(),
                Vector3::y(),
                Vector3::y(),
                Vector3::y(),
            ]);
            vbo.set_basic_typed_vertex_attrib_pointer::<f32>(1, 3, gl::FLOAT, false);
            vbo.set_vertex_attrib_enabled(1, true);
        })
        // Texture Coordinates
        .add_vbo(|vbo| {
            vbo.copy_data_static::<Vector2<f32>>(&[
                Vector2::new(0.0, tex_scale),
                Vector2::new(tex_scale, tex_scale),
                Vector2::new(tex_scale, 0.0),
                Vector2::new(0.0, 0.0),
            ]);
            vbo.set_basic_typed_vertex_attrib_pointer::<f32>(2, 2, gl::FLOAT, false);
            vbo.set_vertex_attrib_enabled(2, true);
        })
        .set_primitive_mode(gl::TRIANGLES)
        .set_indices(&[0, 1, 2, 0, 2, 3])
        .set_material(material)
        .build()
}
