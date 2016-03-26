extern crate glium;

use glium::program::Program;
use std::rc::Rc;

trait DisplayObject {

}

// TODO: rotation quaternion
pub struct DisplayInfo {
	pub x:f32,
	pub y:f32,
	pub z:f32,
	pub shader:Option<Rc<Program>>
}