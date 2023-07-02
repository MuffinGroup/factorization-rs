#version 140

in vec2 my_attr;
out vec4 color;

void main() {
    color = vec4(my_attr, 1.0, 1.0);
}