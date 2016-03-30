use math3d::matrix::Matrix4x4;

// a matrix stack to store and pop transformations
// the matrix stack needs to own its matrices, either through copying or moving

pub struct MatrixStack {
	stack:Vec<Matrix4x4<f32>>,
}

impl MatrixStack {
	// initialization with a identity shader
	pub fn new() -> MatrixStack {
		let mut initial = Vec::new();
		initial.push(Matrix4x4::<f32> {
		m11:1f32,m21:0f32,m31:0f32,m41:0f32,
		m12:0f32,m22:1f32,m32:0f32,m42:0f32,
		m13:0f32,m23:0f32,m33:1f32,m43:0f32,
		m14:0f32,m24:0f32,m34:0f32,m44:1f32,
		});
		MatrixStack { stack:initial }
	}
}