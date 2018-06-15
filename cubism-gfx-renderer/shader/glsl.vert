#version 400

in vec3 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
out float opacity;

uniform mat4 matrix;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = matrix * vec4(position.xy, 0.0, 1.0);
    opacity = position.z;
}