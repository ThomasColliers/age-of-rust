#version 430

in vec3 position;

void main(){
	gl_Position = vec4(position[0], position[1], 0.0, 1.0);
}