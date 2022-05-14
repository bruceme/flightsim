#version 150

in vec3 position;
in vec3 normal;
in vec2 tex;

out vec2 frag_tex;
out vec3 frag_normal;
out vec3 frag_position;

uniform float time;
uniform mat4 view;
uniform mat4 transformation;

void main(void)
{
	frag_tex = tex * 30;
    frag_normal = normal;
	frag_position = position;

	gl_Position = view * transformation * vec4(position, 1.0);
}
