#version 140

out vec4 color;
in vec3 color_attribute;

void main() {
    color = vec4(color_attribute, 1.0);
}