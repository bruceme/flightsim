#version 150


in vec3 position;
in vec3 normal;
in vec2 tex;

out vec2 frag_tex;
out vec3 frag_normal;

uniform mat4 view;
//uniform mat4 projection;

void main(void)
{
	frag_tex = tex;
    frag_normal = normal;

	gl_Position = view * vec4(position, 1.0);
}
