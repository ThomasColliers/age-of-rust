use math3d::matrix::Matrix4x4;
use num::{Float,NumCast,cast,Zero,One};
use std::f32;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Frustum<N: Float> {
	pub projection_matrix:Matrix4x4<N>
}

// TODO: number casting might not be ideal for performance, maybe use traits to improve this
impl<N: Float+NumCast+Zero+One> Frustum<N> {
	pub fn new() -> Frustum<N> {
		Frustum {
			projection_matrix:Matrix4x4::<N>::identity()
		}
	}
	pub fn set_perspective(&mut self, fov:N, aspect_ratio:N, near:N, far:N){
        // calculate the near clipping plane
	let nymax = near * (fov * cast(f32::consts::PI).unwrap() / cast(360).unwrap()).tan();
        let nymin = -nymax;
        let nxmin = nymin * aspect_ratio;
        let nxmax = -nxmin;

        // update the projection matrix (creating new one right now)
        let zero = N::zero();
        let one = N::one();
        let two = one + one;
        self.projection_matrix = Matrix4x4::<N>::identity();
        self.projection_matrix.m11 = (two * near) / (nxmax - nxmin);
        self.projection_matrix.m22 = (two * near) / (nymax - nymin);
        self.projection_matrix.m31 = (nxmax + nxmin) / (nxmax - nxmin);
        self.projection_matrix.m32 = (nymax + nymin) / (nymax - nymin);
        self.projection_matrix.m33 = -((far + near) / (far - near));
        self.projection_matrix.m34 = -one;
        self.projection_matrix.m43 = -((two * far * near) / (far - near));
        self.projection_matrix.m44 = zero;
	}
	pub fn set_orthographic(&mut self, xmin:N, xmax:N, ymin:N, ymax:N, zmin:N, zmax:N){
        let zero = N::zero();
        let one = N::one();
        let two = one + one;
        self.projection_matrix = Matrix4x4::<N>::identity();
        self.projection_matrix.m11 = two / (xmax - xmin);
        self.projection_matrix.m22 = two / (ymax - ymin);
        self.projection_matrix.m33 = -two / (zmax - zmin);
        self.projection_matrix.m41 = -((xmax + xmin) / (xmax - xmin));
        self.projection_matrix.m42 = -((ymax + ymin) / (ymax - ymin));
        self.projection_matrix.m43 = -((zmax + zmin) / (zmax - zmin));
        self.projection_matrix.m44 = one;
	}
}