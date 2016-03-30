extern crate num;

use num::{Num};

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Matrix1x1<N> {
	pub m11:N,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Matrix2x2<N>{
	pub m11:N, pub m21:N,
	pub m12:N, pub m22:N,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Matrix3x3<N> {
	pub m11:N, pub m21:N, pub m31:N,
	pub m12:N, pub m22:N, pub m32:N,
	pub m13:N, pub m23:N, pub m33:N,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Matrix4x4<N> {
	pub m11:N, pub m21:N, pub m31:N, pub m41:N,
	pub m12:N, pub m22:N, pub m32:N, pub m42:N,
	pub m13:N, pub m23:N, pub m33:N, pub m43:N,
	pub m14:N, pub m24:N, pub m34:N, pub m44:N,
}

// TODO: make this into a macro so I can easily generate this
impl<N: Num> Matrix4x4<N> {
	pub fn identity() -> Matrix4x4<N> {
		Matrix4x4::<N> {
			m11:num::one(),m21:num::zero(),m31:num::zero(),m41:num::zero(),
			m12:num::zero(),m22:num::one(),m32:num::zero(),m42:num::zero(),
			m13:num::zero(),m23:num::zero(),m33:num::one(),m43:num::zero(),
			m14:num::zero(),m24:num::zero(),m34:num::zero(),m44:num::one(),
		}
	}
}