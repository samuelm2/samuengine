#version 140


out vec4 color;
void main() {
    color = vec4(gl_FragCoord.x, gl_FragCoord.y, gl_FragCoord.z, 1.0);
}