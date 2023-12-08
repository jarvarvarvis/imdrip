#version 330 core

layout (location = 0) in vec2 tex_coord;

out vec4 vertex_tex_coord;

void main() {
    gl_Position = vec4(tex_coord, 0.0, 1.0);
    vertex_tex_coord = tex_coord;
}
