extern crate glium;

use std::collections::HashMap;
use glium::program::Program;
use glium::backend::Facade;
use std::marker::PhantomData;

pub struct ShaderManager {
	shaders:HashMap<String,Program>,
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

	pub fn load(&self, vertex: &str, fragment: &str) -> Result<&Program, &str> {

		Err("Test")
	}
}

struct ShaderSource {
	vertex:String,
	fragment:String,
	geometry:Option<String>,
}