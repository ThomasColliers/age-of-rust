extern crate num;
use num::Num;

// Some macro's to create the implementations for all the fixed-size matrices

// create matrix with zero values
macro_rules! matrix_new(
	($t: ident, $($index: ident),+) => (
		impl<N: Num> $t<N> {
			#[inline]
			pub fn new() -> $t<N> {
				$t {
					$($index: num::zero()),+
				}
			}
		}
	)
);

// create identity matrix
macro_rules! matrix_identity(
	($t: ident, $($diag_index: ident),+) => (
		impl<N: Num> $t<N> {
			#[inline]
			pub fn identity() -> $t<N> {
				let mut mat: $t<N> = $t::new();
				$(mat.$diag_index = num::one();)+
				mat
			}
		}
	)
);

// 1x1 Matrix

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Matrix1x1<N> {
	pub m11:N,
}

matrix_new!(Matrix1x1,m11);
matrix_identity!(Matrix1x1,m11);

// 2x2 Matrix

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Matrix2x2<N>{
	pub m11:N, pub m21:N,
	pub m12:N, pub m22:N,
}

matrix_new!(Matrix2x2,m11,m12,m21,m22);
matrix_identity!(Matrix2x2,m11,m22);

// 3x3 Matrix

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Matrix3x3<N> {
	pub m11:N, pub m21:N, pub m31:N,
	pub m12:N, pub m22:N, pub m32:N,
	pub m13:N, pub m23:N, pub m33:N,
}

matrix_new!(Matrix3x3,m11,m21,m31,m12,m22,m32,m13,m23,m33);
matrix_identity!(Matrix3x3,m11,m22,m33);

// 4x4 Matrix

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Matrix4x4<N> {
	pub m11:N, pub m21:N, pub m31:N, pub m41:N,
	pub m12:N, pub m22:N, pub m32:N, pub m42:N,
	pub m13:N, pub m23:N, pub m33:N, pub m43:N,
	pub m14:N, pub m24:N, pub m34:N, pub m44:N,
}

matrix_new!(Matrix4x4,m11,m21,m31,m41,m12,m22,m32,m42,m13,m23,m33,m43,m14,m24,m34,m44);
matrix_identity!(Matrix4x4,m11,m22,m33,m44);