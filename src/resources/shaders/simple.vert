#version 140
in vec3 position;
in vec3 normal;

uniform mat4 model_view_perspective_matrix;

out vec3 f_normal;

void main() {
    f_normal = normal;
    gl_Position = model_view_perspective_matrix * vec4(position, 1.0);
    // gl_Position = vec4(position, 1.0);
}