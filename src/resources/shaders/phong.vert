#version 140
in vec3 position;
in vec3 normal;

uniform mat4 model_view;
uniform mat4 perspective;

out vec3 f_normal;
out vec4 f_position;

void main() {
    f_normal = normal;
    vec4 mv_position = model_view * vec4(position, 1.0);
    f_position = mv_position;
    gl_Position = perspective * mv_position; 
    // gl_Position = vec4(position, 1.0);
}