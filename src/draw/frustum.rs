use math3d::matrix::Matrix4x4;
use num::Num;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Frustum<N: Num> {
	pub projection_matrix:Matrix4x4<N>
}

impl<N: Num> Frustum<N> {
	pub fn new() -> Frustum<N> {
		Frustum {
			projection_matrix:Matrix4x4::<N>::identity()
		}
	}
	pub fn set_perspective(&mut self){
		
	}
	pub fn set_orthographic(&mut self){

	}
}