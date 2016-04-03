extern crate num;
use num::Num;

macro_rules! vector_new(
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

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Vector1<N> {
	pub x:N,
}

vector_new!(Vector1,x);

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Vector2<N> {
	pub x:N,
	pub y:N,
}

vector_new!(Vector2,x,y);

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Vector3<N> {
	pub x:N,
	pub y:N,
	pub z:N,
}

vector_new!(Vector3,x,y,z);

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Vector4<N> {
	pub x:N,
	pub y:N,
	pub z:N,
	pub w:N,
}

vector_new!(Vector4,x,y,z,w);