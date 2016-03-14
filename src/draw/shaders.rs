extern crate glium;

use std::collections::HashMap;
use glium::program::Program;
use glium::backend::Facade;

pub struct ShaderManager {
	shaders:HashMap<String,Program>
}

impl ShaderManager {
	pub fn new<F>(display: &F) -> ShaderManager where F: Facade + Clone {
		ShaderManager {
			shaders:HashMap::new()
		}
	}
}