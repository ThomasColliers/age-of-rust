use na::{Mat4,Eye};

// a matrix stack to store and pop transformations
// the matrix stack needs to own its matrices, either through copying or moving

pub struct MatrixStack {
	stack:Vec<Mat4<f32>>,
}

impl MatrixStack {
	// initialization with a identity shader
	pub fn new() -> MatrixStack {
		let mut initial = Vec::with_capacity(5);
		initial.push(Mat4::<f32>::new_identity(4));
		MatrixStack { stack:initial }
	}

	pub fn load_identity(&mut self){
		let len = self.stack.len();
		self.stack[len-1] = Mat4::<f32>::new_identity(4);
	}
	pub fn load_matrix(&mut self, mtrx:Mat4<f32>){
		let len = self.stack.len();
		self.stack[len-1] = mtrx;
	}
	pub fn get_matrix(&mut self) -> &Mat4<f32> {
		return &self.stack.last();
	}
	pub fn push(&mut self){
		let mut new = self.stack.last().unwrap().clone();
		self.stack.push(new);
	}
	pub fn pop(&mut self){
		self.stack.pop();
	}
}