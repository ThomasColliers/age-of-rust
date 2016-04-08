extern crate glium;

use num::{Num,NumCast,cast,Float,Zero,One};

use glium::program::Program;
use na::{Vec3,Mat4,cross,Norm,BaseFloat};
use std::rc::Rc;

pub trait HasFrame<T: BaseFloat> {
	fn get_frame(&self) -> &Frame<T>;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Frame<T: BaseFloat> {
	origin:Vec3<T>,
	forward:Vec3<T>,
	up:Vec3<T>,
}

impl<T: Float+Num+NumCast+Zero+One+BaseFloat> Frame<T> {
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

	pub fn look_at(&mut self, x:T, y:T, z:T){
		let mut target = Vec3::<T>::new(x,y,z);
		target = target - self.origin;
		let mut projected_target = target.clone();

		// project on the xz plane or yz plane
		if target.x.abs() < cast(0.00001).unwrap() && target.z.abs() < cast(0.00001).unwrap() { // yz
			projected_target.x = cast(0.0).unwrap();
			projected_target.normalize_mut();

			self.forward.x = cast(0.0).unwrap(); self.forward.y = cast(0.0).unwrap(); self.forward.z = cast(-1.0).unwrap();

			self.up = cross(&self.forward,&projected_target);
			self.forward = cross(&self.up,&target);

			self.forward = -self.forward;

			self.up.normalize_mut();
		}else{ // xz
			projected_target.y = cast(0.0).unwrap()
;			projected_target.normalize_mut();

			self.forward = target.clone();
			self.forward.normalize_mut();
			self.up.x = cast(0.0).unwrap(); self.up.y = cast(1.0).unwrap(); self.up.z = cast(0.0).unwrap();
			let mut right = cross(&projected_target, &self.up);
			right = -right;
			self.up = cross(&target,&right);
		}
	}

	// matrix has no translation information and is transposed
	pub fn get_camera_matrix(&mut self) -> Mat4<T> {
		
	}
}

pub trait Drawable {
	fn draw(&self,target:&mut glium::Frame,params:&glium::DrawParameters, mvp_matrix:&Mat4<f32>);
}