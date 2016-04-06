#[macro_use]
extern crate glium;

extern crate nalgebra as na;
extern crate rand;
extern crate num;

mod world;
mod draw;

use world::terrain::Terrain;
use draw::shaders::ShaderManager;
use draw::matrix_stack::MatrixStack;
use draw::display_object::Drawable;
use na::PerspMat3;


fn main() {
	use glium::{DisplayBuild, Surface};
	// can not do multisampling due to glutin bug with_multisampling(16)
	let display = glium::glutin::WindowBuilder::new().with_vsync().with_depth_buffer(24).build_glium().unwrap();

	// draw parametres
	let params = glium::DrawParameters {
		depth: glium::Depth {
			test:glium::DepthTest::IfLess,
			write:true,
			.. Default::default()
		},
		.. Default::default()
	};

	// set up the matrix stacks
	let mut model_view_matrix = MatrixStack::new();
	let mut projection_matrix = MatrixStack::new();

	// view frustum
	let display_size = display.get_window().unwrap().get_inner_size_pixels().unwrap();
	let mut frustum = PerspMat3::<f32>::new(display_size.0 as f32/display_size.1 as f32, 35f32, 1f32, 5000f32);
	// update the projection matrix
	projection_matrix.load_matrix(frustum.as_mat().clone());

	// setup the shaders
	let mut shader_manager = ShaderManager::new(&display);

	// create terrain
	let terrain = Terrain::new(&display,&mut shader_manager,5);

    // listen for events produced in the window and wait to be received
    let mut t:f32 = 0.0;
    loop {
    	t += 0.002;
    	if t > 0.5 {
    		t = -0.5;
    	}
    	// get reference to the frame
    	let mut target = display.draw();

    	// clear the background
    	target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0),1.0);

    	// draw the terrain

    	terrain.draw(&mut target,&params,t);
    	//target.draw(terrain.get_vertex_buffer(),terrain.get_index_buffer(),terrain.get_shader(),&glium::uniforms::EmptyUniforms,&params).unwrap();

    	//target.draw(&vertex_buffer,&indices,&program,&glium::uniforms::EmptyUniforms,&params).unwrap();

    	// finish drawing and destroy frame object
    	target.finish().unwrap();

	    for ev in display.poll_events(){
	    	match ev {
	    		glium::glutin::Event::Closed => return,
	    		glium::glutin::Event::Resized(width, height) => {
	    			// update the view frustum and projection matrix
	    			frustum.set_aspect(width as f32/height as f32);
	    			projection_matrix.load_matrix(frustum.as_mat().clone());
	    		},
	    		_ => ()
	    	}
	    }
    }
}