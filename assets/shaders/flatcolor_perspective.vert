#version 430

in vec3 position;
in vec2 tex_coords;

uniform mat4 mvpMatrix;

void main(void){
	//gl_Position = vec4(position[0], position[1], 0.0, 1.0);
	gl_Position = mvpMatrix * vec4(position[0], position[1], position[2], 1.0);
}