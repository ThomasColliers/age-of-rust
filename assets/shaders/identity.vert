#version 430

in vec3 position;

uniform float t;

void main(){
	vec3 pos = position;
	pos[0] += t;
	gl_Position = vec4(pos[0], pos[1], 0.0, 1.0);
}