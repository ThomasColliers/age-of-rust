extern crate glium;
extern crate num;

use std::fmt::Display;
use num::{Num,NumCast,cast,Float,Zero,One};

use glium::program::Program;
use cgmath::{Matrix4,Vector3,BaseFloat};
use std::rc::Rc;

pub trait HasFrame<T: BaseFloat> {
	fn get_frame(&self) -> &Frame<T>;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Frame<T: BaseFloat> {
	origin:Vector3<T>,
	forward:Vector3<T>,
	up:Vector3<T>,
}

impl<T: Float+Num+NumCast+Zero+One+BaseFloat+Display> Frame<T> {
	pub fn new() -> Frame<T> {
		// TODO: use quaternion instead
		Frame::<T> {
			origin:Vector3::<T>::zero(),
			forward:Vector3::<T>::unit_z(),
			up:Vector3::<T>::unit_y(),
		}
	}

	pub fn set_origin(&mut self, x:T, y:T, z:T){
		self.origin = Vector3::<T>::new(x,y,y);
	}
	
	pub fn get_origin(&self) -> &Vector3<T> {
		&self.origin
	}
}

pub trait Drawable {
	fn draw(&self,target:&mut glium::Frame,params:&glium::DrawParameters, mvp_matrix:&Matrix4<f32>);
}