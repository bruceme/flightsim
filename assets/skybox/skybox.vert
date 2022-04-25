#version 150


in vec3 vertices;
in vec3 normals;
in vec2 texies;

out vec2 frag_texies;
out vec3 frag_normals;

uniform mat4 view;
uniform mat4 projection;

void main(void)
{
	frag_texies = texies;

	gl_Position =  projection * view * vec4(vertices, 1.0);
}
