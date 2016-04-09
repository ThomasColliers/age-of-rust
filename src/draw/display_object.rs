extern crate glium;
extern crate num;

use std::fmt::Display;
use num::{Num,NumCast,cast,Float,Zero,One};

use glium::program::Program;
use na::{Vec3,Mat4,cross,Norm,BaseFloat,Eye};
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

impl<T: Float+Num+NumCast+Zero+One+BaseFloat+Display> Frame<T> {
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
			projected_target.x = num::zero();
			projected_target.normalize_mut();

			self.forward.x = num::zero(); self.forward.y = num::zero(); self.forward.z = cast(-1.0).unwrap();

			self.up = cross(&self.forward,&projected_target);
			self.forward = cross(&self.up,&target);

			self.forward = -self.forward;

			self.up.normalize_mut();
		}else{ // xz
			projected_target.y = num::zero();
			projected_target.normalize_mut();

			self.forward = target.clone();
			self.forward.normalize_mut();
			self.up.x = num::zero(); self.up.y = num::one(); self.up.z = num::zero();
			let mut right = cross(&projected_target, &self.up);
			right = -right;
			self.up = cross(&target,&right);
		}
	}

	// matrix has no translation information and is transposed
	pub fn get_camera_matrix(&mut self, rotation_only:bool) -> Mat4<T> {
		let z = -self.forward;
		let x = cross(&self.up, &z);

		// build the rotation matrix
		let mut mat:Mat4<T> = Mat4::new_identity(4);
		mat.m11 = x.x;
		mat.m12 = x.y;
		mat.m13 = x.z;
		mat.m14 = num::zero();
		mat.m21 = self.up.x;
		mat.m22 = self.up.y;
		mat.m23 = self.up.z;
		mat.m24 = num::zero();
		mat.m31 = z.x;
		mat.m32 = z.y;
		mat.m33 = z.z;
		mat.m34 = num::zero();
		mat.m41 = num::zero();
		mat.m42 = num::zero();
		mat.m43 = num::zero();
		mat.m44 = num::one();

		if !rotation_only {
			// create translation matrix
			let mut translation:Mat4<T> = Mat4::new_identity(4);
			translation.m14 = -self.origin.x;
			translation.m24 = -self.origin.y;
			translation.m34 = -self.origin.z;
			// multiply it with our current rotaton only matrix
			mat = mat * translation;
		}

		mat
	}
}

pub trait Drawable {
	fn draw(&self,target:&mut glium::Frame,params:&glium::DrawParameters, mvp_matrix:&Mat4<f32>);
}