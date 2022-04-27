#version 150


in vec3 vertices;
in vec3 normals;
in vec2 texies;

out vec2 frag_texies;
out vec3 frag_normals;

uniform mat4 view;
//uniform mat4 projection;

void main(void)
{
	frag_texies = texies;
    frag_normals = normals;

	gl_Position = view * vec4(vertices, 1.0);
}
