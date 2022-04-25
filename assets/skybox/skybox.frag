#version 150

in vec2 frag_texies;
in vec3 frag_normals;
out vec4 out_color;

uniform sampler2D tex;

void main(void)
{
	out_color = texture(tex, frag_texies);
}
