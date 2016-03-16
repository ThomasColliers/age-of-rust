extern crate glium;

use std::collections::HashMap;
use glium::program::Program;
use glium::backend::Facade;

pub struct ShaderManager {
	shaders:HashMap<String,Program>
}

/*
 The shader manager should load and cache files and compile the shaders.
 By doing this, display objects will be able to retrieve their shaders from
 the manager. Every scene object should have a shader assigned to it as well
 as other basic properties such as position
*/

impl ShaderManager {
	pub fn new<F>(display: &F) -> ShaderManager where F: Facade + Clone {
		ShaderManager {
			shaders:HashMap::new()
		}
	}
}

struct ShaderSource {
	vertex:String,
	fragment:String,
	geometry:Option<String>,
}


/* DEFINE SOME SHADER PROGRAMS */
/*
let identity_vertex:String = String::from(r#"
	#version 430

	in vec2 position;

	void main(){
		gl_Position = vec4(position, 0.0, 1.0);
	}
"#);

const identity_fragment:String = String::from(r#"
	#version 430

	out vec4 color;

	void main(){
		color = vec4(1.0, 0.0, 0.0, 1.0);
	}
"#);*/