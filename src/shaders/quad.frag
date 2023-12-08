#version 330 core

in vec2 vertex_tex_coord;

uniform sampler2D image_texture;

void main() {
    gl_FragColor = texture(image_texture, vertex_tex_coord);
}
