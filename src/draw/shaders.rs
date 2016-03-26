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
	shaders:HashMap<String,Program>
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
}

struct ShaderSource {
	vertex:String,
	fragment:String,
	geometry:Option<String>,
}