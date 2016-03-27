#[macro_use]
extern crate glium;
extern crate rand;

mod world;
use world::terrain::Terrain;

mod draw;
use draw::shaders::ShaderManager;

mod math3d;

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

	// build all the shaders
	let mut shader_manager = ShaderManager::new(&display);

	// create terrain
	let mut terrain = Terrain::new(&display,&mut shader_manager,5);

    // listen for events produced in the window and wait to be received
    loop {
    	// get reference to the frame
    	let mut target = display.draw();

    	// clear the background
    	target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0),1.0);

    	// draw the terrain
    	terrain.draw(&target);
    	//target.draw(terrain.get_vertex_buffer(),terrain.get_index_buffer(),terrain.get_shader(),&glium::uniforms::EmptyUniforms,&params).unwrap();

    	//target.draw(&vertex_buffer,&indices,&program,&glium::uniforms::EmptyUniforms,&params).unwrap();

    	// finish drawing and destroy frame object
    	target.finish().unwrap();

	    for ev in display.poll_events(){
	    	match ev {
	    		glium::glutin::Event::Closed => return,
	    		_ => ()
	    	}
	    }
    }
}