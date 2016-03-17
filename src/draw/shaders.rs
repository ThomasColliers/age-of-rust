extern crate glium;

use std::error::Error;
use std::collections::HashMap;
use glium::program::Program;
use glium::backend::Facade;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

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

	pub fn load<F>(&mut self, display: &F, vertex: &str, fragment: &str) -> Result<&Program, &str> where F: Facade + Clone {
		// load the shader files
		let vertex_shader_src = self.loadfile(vertex);
		let fragment_shader_src = self.loadfile(fragment);

		// load them into the program and return it
		let program = glium::Program::from_source(display, &vertex_shader_src[..], &fragment_shader_src[..], None).unwrap();

		// temporary
		let refer = &program;
		// assign them to the hashmap
		let mut key = vertex.to_owned();
		key.push_str(";");
		key.push_str(fragment);
		self.shaders.insert(key,program);

		Ok(&refer)
	}

	fn loadfile(&self, fname: &str) -> String {
		// build the path
		let mut path = PathBuf::from("./assets/shaders");
		path.push(fname);

		// load the file
		let mut file = match File::open(&path){
			Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
			Ok(file) => file,
		};

		// read the contents into a string
		let mut s = String::new();
		match file.read_to_string(&mut s){
			Err(why) => panic!("couldn't read {}: {}", path.display(), Error::description(&why)),
			Ok(_) => s
		}
	}
}

struct ShaderSource {
	vertex:String,
	fragment:String,
	geometry:Option<String>,
}