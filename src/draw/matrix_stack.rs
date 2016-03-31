use math3d::matrix::Matrix4x4;

// a matrix stack to store and pop transformations
// the matrix stack needs to own its matrices, either through copying or moving

pub struct MatrixStack {
	stack:Vec<Matrix4x4<f32>>,
}

impl MatrixStack {
	// initialization with a identity shader
	pub fn new() -> MatrixStack {
		let mut initial = Vec::with_capacity(5);
		initial.push(Matrix4x4::<f32>::identity());
		MatrixStack { stack:initial }
	}
}