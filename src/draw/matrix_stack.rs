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

	pub fn load_identity(&mut self){
		let len = self.stack.len();
		self.stack[len] = Matrix4x4::<f32>::identity();
	}
	pub fn load_matrix(&mut self, mtrx:Matrix4x4<f32>){
		self.stack.push(mtrx);
	}
	pub fn mult_matrix(&mut self){

	}
	pub fn push(&mut self){
		let mut new = self.stack.last().unwrap().clone();
		self.stack.push(new);
	}
	pub fn pop(&mut self){
		self.stack.pop();
	}
}