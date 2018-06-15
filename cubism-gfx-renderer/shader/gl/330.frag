#version 330

in vec4 v_color;
in vec2 v_tex_coord;
uniform sampler2D tex;
out vec4 Target0;

void main() {
    Target0 = texture(tex, v_tex_coord);
}
