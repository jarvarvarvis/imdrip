#version 330 core

in vec2 vertex_tex_coord;

uniform ivec2 window_size;
uniform sampler2D image_texture;

out vec4 frag_color;

const vec3 darker_grid_color = vec3(0.3);
const vec3 lighter_grid_color = vec3(0.7);

void main() {
    vec4 sampled_color = texture(image_texture, vertex_tex_coord);

    ivec2 window_pos = ivec2(window_size * vertex_tex_coord);

    // Calculate grid
    const float min_tile_size = 15.0;

    vec2 size_fit = window_size / min_tile_size;
    vec2 grid_size = floor(size_fit);
    vec2 uv = fract(vertex_tex_coord * (grid_size * 0.5)) - 0.5;
    float grid_mix = step(uv.x * uv.y, 0.0);
    vec3 grid_color = mix(darker_grid_color, lighter_grid_color, grid_mix);

    // Calculate final color
    vec3 final_color = mix(grid_color, sampled_color.rgb, sampled_color.a);
    frag_color = vec4(final_color, 1.0);
}
