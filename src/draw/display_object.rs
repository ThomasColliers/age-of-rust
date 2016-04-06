extern crate glium;

use num::{Num,NumCast,Float,Zero,One};

use glium::program::Program;
use na::{Vec3};
use std::rc::Rc;

pub trait HasFrame<T> {
	fn get_frame(&self) -> &Frame<T>;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Frame<T> {
	origin:Vec3<T>,
	forward:Vec3<T>,
	up:Vec3<T>,
}

impl<T: Float+Num+NumCast+Zero+One> Frame<T> {
	pub fn new() -> Frame<T> {
		// TODO: use quaternion instead
		Frame::<T> {
			origin:Vec3::<T>::zero(),
			forward:Vec3::<T>::z(),
			up:Vec3::<T>::y(),
		}
	}

	pub fn set_origin(&mut self, x:T, y:T, z:T){
		self.origin = Vec3::<T> {
			x:x,y:y,z:z
		}
	}
	
	pub fn get_origin(&self) -> &Vec3<T> {
		&self.origin
	}
}

pub trait Drawable {
	fn draw(&self,target:&mut glium::Frame,params:&glium::DrawParameters,t:f32);
}