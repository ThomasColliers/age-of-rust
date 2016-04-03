extern crate glium;
use num::{Num,NumCast,Float,Zero,One};

use glium::program::Program;
use math3d::vector::Vector3;
use std::rc::Rc;

trait HasFrame<T> {
	fn frame(&self) -> &Frame<T>;
}

pub struct Frame<T> {
	origin:Vector3<T>,
	forward:Vector3<T>,
	up:Vector3<T>,
}

impl<T: Float+Num+NumCast+Zero+One> Frame<T> {
	pub fn new() -> Frame<T> {
		// TODO: initialise with different forward and up vectors
		Frame::<T> {
			origin:Vector3::<T>::new(),
			forward:Vector3::<T> {
				x:T::zero(),
				y:T::zero(),
				z:T::one(),
			},
			up:Vector3::<T> {
				x:T::zero(),
				y:T::one(),
				z:T::zero(),
			},
		}
	}

	pub fn set_origin(&mut self, x:T, y:T, z:T){
		self.origin = Vector3::<T> {
			x:x,y:y,z:z
		}
	}
	
	pub fn get_origin(&self) -> &Vector3<T> {
		&self.origin
	}
}

trait Drawable {
	fn draw(&self);
}

/*
// TODO: rotation quaternion
pub struct DisplayInfo {
	pub x:f32,
	pub y:f32,
	pub z:f32,
	pub shader:Option<Rc<Program>>
}
*/