#version 140
in vec3 f_normal;
in vec4 f_position;

uniform mat3 normal_matrix;
uniform vec3 light_position;

out vec4 color;
void main() {
    vec4 lightColor = vec4(1.0, 1.0, 1.0, 1.0);
    vec4 objectColor = vec4(1.0, 0.0, 0.0, 1.0);

    vec3 normal = vec3(normalize(normal_matrix * normalize(f_normal)));
    vec3 lightToPos = normalize(light_position - f_position.xyz);
    float diffuseScalar = max(0.0, dot(lightToPos, normal)); 
    
    vec3 eyeToPos = normalize(f_position.xyz);
    vec3 reflection = reflect(lightToPos, normal);
    float specularScalar = clamp(dot(reflection, eyeToPos), 0.0, 1.0);
    vec4 specularColor = vec4(pow(specularScalar, 10));

    color = clamp(diffuseScalar * objectColor + specularColor, vec4(0.0), vec4(1.0));
}