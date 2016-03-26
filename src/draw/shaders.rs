extern crate glium;

use std::collections::HashMap;
use glium::program::Program;
use glium::backend::Facade;

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

use std::rc::Rc;

/*
 The shader manager loads and caches shaders
*/

pub struct ShaderManager {
	shaders:HashMap<String,Rc<Program>>
}

impl ShaderManager {
	pub fn new<F>(display: &F) -> ShaderManager where F: Facade + Clone {
		ShaderManager {
			shaders:HashMap::new(),
		}
	}

	fn loadfile(&self, fname: &str) -> Result<String, String> {
		// build the path
		let mut path = PathBuf::from("./assets/shaders");
		path.push(fname);

		let mut file_handle = match File::open(&path){
			Err(why) => return Err(format!("Failed to open {}: {}.",path.display(), Error::description(&why))),
			Ok(file_handle) => file_handle,
		};

		let mut s = String::new();
		match file_handle.read_to_string(&mut s){
			Err(why) => return Err(format!("Couldn't read {}: {}",path.display(), Error::description(&why))),
			Ok(_) => Ok(s)
		}
	}

	pub fn load<F>(&mut self, display: &F, vertex: &str, fragment: &str) -> Result<Rc<Program>, String> where F: Facade + Clone {
		// build the key
		let mut key = vertex.to_owned();
		key.push_str(";");
		key.push_str(fragment);

		// check if it's loaded and compiled already
		if self.shaders.contains_key(&key) {
			return Ok(self.shaders.get(&key).unwrap().clone())
		}

		// open and read the files
		let vertex_shader_src = match self.loadfile(vertex){
			Err(why) => return Err(why),
			Ok(shader) => shader
		};
		let fragment_shader_src = match self.loadfile(fragment){
			Err(why) => return Err(why),
			Ok(shader) => shader
		};

		// load them into the program
		let program = match glium::Program::from_source(display, &vertex_shader_src[..], &fragment_shader_src[..], None){
			Err(why) => return Err(format!("Couldn't compile shader: {}",Error::description(&why))),
			Ok(prog) => prog
		};
		let program = Rc::new(program);

		// move the program into our hashmap
		self.shaders.insert(key,program.clone());

		Ok(program.clone())
	}
}

struct ShaderSource {
	vertex:String,
	fragment:String,
	geometry:Option<String>,
}