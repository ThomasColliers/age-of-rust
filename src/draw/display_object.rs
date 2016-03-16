extern crate glium;

use glium::program::Program;

trait DisplayObject {

}

// TODO: rotation quaternion
pub struct DisplayInfo<'a> {
	pub x:f32,
	pub y:f32,
	pub z:f32,
	pub shader:Option<&'a Program>
}