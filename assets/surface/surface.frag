#version 150

in vec2 frag_tex;
in vec3 frag_normal;
out vec4 out_color;

uniform sampler2D texture0;

void main(void)
{
	out_color = texture(texture0, frag_tex);
}
