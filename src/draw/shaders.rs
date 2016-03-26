extern crate glium;

use std::collections::HashMap;
use glium::program::Program;
use glium::backend::Facade;

use std::io::prelude::*;
use std::fs::File;

/*
 The shader manager loads and caches shaders
*/

pub struct ShaderManager {
	shaders:HashMap<String,Program>,
}

impl ShaderManager {
	pub fn new<F>(display: &F) -> ShaderManager where F: Facade + Clone {
		ShaderManager {
			shaders:HashMap::new()
		}
	}

	pub fn load(&self, vertex: &str, fragment: &str) -> Result<&Program, &str> {
		// open and read the vertex file
		let mut vertex_file = match File::open(vertex){
			Ok(vertex_file) => vertex_file,
			Err(..) => return Err("Unable to open file."),
		};
		let mut vertex_data = String::new();
		match vertex_file.read_to_string(&mut vertex_data){
			Err(..) => return Err("Failed to read file."),
			_ => {}
		};
		/*let vertex_file = try!(File::open(vertex));

		let mut vertex_data = String::new();
		try!(vertex_file.read_to_string(&mut vertex_data));*/
		// open and read the fragment file
		/*let fragment_file = try!(File::open(fragment));
		let mut fragment_data = String::new();
		try!(fragment_file.read_to_string(&mut fragment_data));*/


		// try and open
		Err("Test")
	}

/*
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
*/

struct ShaderSource {
	vertex:String,
	fragment:String,
	geometry:Option<String>,
}