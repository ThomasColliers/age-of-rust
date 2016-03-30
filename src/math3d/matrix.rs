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