#version 140

in vec2 position;
in vec3 rgb;
out vec3 color_attribute;

uniform mat4 matrix;

void main() {
    color_attribute = rgb;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}