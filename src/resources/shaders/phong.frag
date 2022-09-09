#version 140
in vec3 f_normal;
in vec4 f_position;

uniform mat3 normal_matrix;

out vec4 color;
void main() {
    vec3 lightPos = vec3(1.0, 1.0, 0.0);
    vec4 lightColor = vec4(1.0, 1.0, 1.0, 1.0);
    vec4 objectColor = vec4(1.0, 0.0, 0.0, 1.0);

    vec3 normal = vec3(normalize(normal_matrix * normalize(f_normal)));
    vec3 lightToPos = normalize(lightPos - f_position.xyz);

    float lightScalar = max(0.0, dot(lightToPos, normal)); 

    color = lightScalar * objectColor;
}