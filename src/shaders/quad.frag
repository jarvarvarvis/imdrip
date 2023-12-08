#version 330 core

in vec2 vertex_tex_coord;

uniform sampler2D image_texture;

out vec4 frag_color;

void main() {
    frag_color = texture(image_texture, vertex_tex_coord);
}
