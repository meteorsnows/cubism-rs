#version 330

in vec2 a_pos;
in vec2 a_tex_coord;
in vec3 a_color;

uniform mat4 u_mvp;

out vec4 v_color;
out vec2 v_tex_coord;

void main() {
    v_color = vec4(a_color, 1.0);
    v_tex_coord = a_tex_coord;
    gl_Position = vec4(a_pos, 0.0, 1.0) * u_mvp;
}
