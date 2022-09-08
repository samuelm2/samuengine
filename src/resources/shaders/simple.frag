#version 140
in vec3 f_normal;

uniform mat3 normal_matrix;

out vec4 color;
void main() {
    color = vec4(normal_matrix * normalize(f_normal), 1.0);
}