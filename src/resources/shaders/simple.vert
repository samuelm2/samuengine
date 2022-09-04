#version 140
in vec3 position;

uniform mat4 model;


void main() {
    gl_Position = model * vec4(position, 1.0);
    // gl_Position = vec4(position, 1.0);
}