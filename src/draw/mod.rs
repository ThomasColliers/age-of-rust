pub mod shaders;
pub mod display_object;
pub mod matrix_stack;

#[derive(Copy, Clone)]
pub struct Vertex {
	pub position:[f32;3],
	pub texcoords:[f32;2],
	pub normal:[f32;3],
}
implement_vertex!(Vertex, position, texcoords, normal);