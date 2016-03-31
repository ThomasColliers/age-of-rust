use super::matrix_stack::MatrixStack;

pub struct TransformPipeline<'a> {
	modelview_stack:&'a MatrixStack,
	projection_stack:&'a MatrixStack,
}

impl<'a> TransformPipeline<'a> {
	pub fn new(mv_stack:&'a MatrixStack, pj_stack:&'a MatrixStack) -> TransformPipeline<'a> {
		TransformPipeline {
			modelview_stack:mv_stack,
			projection_stack:pj_stack,
		}
	}
}