#version 430

in vec3 position;

uniform mat4 mvpMatrix;

void main(){
	vec4 pos = vec4(position, 1.0);
	gl_Position = mvpMatrix * pos;
}