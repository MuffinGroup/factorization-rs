#version 140

in vec2 position;
in vec2 tex_coords;
in vec3 rgb;
out vec3 color_attribute;
out vec2 v_tex_coords;

uniform mat4 matrix;

void main() {
    color_attribute = rgb;
    v_tex_coords = tex_coords;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}