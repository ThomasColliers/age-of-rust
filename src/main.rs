#[macro_use]
extern crate glium;

extern crate cgmath;
extern crate rand;
extern crate num;
extern crate time;

mod world;
mod draw;

use world::terrain::Terrain;
use draw::shaders::ShaderManager;
use draw::display_object::{Drawable,Frame};
use cgmath::{Matrix4};

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

	// view frustum
	let display_size = display.get_window().unwrap().get_inner_size_pixels().unwrap();
	let mut perspective_matrix:Matrix4<f32> = cgmath::perspective(cgmath::deg(45.0),display_size.0 as f32/display_size.1 as f32,0.0001,100.0);

	// setup the shaders
	let mut shader_manager = ShaderManager::new(&display);

	// create terrain
	let terrain = Terrain::new(&display,&mut shader_manager,5);

    // listen for events produced in the window and wait to be received
    let mut t:u64 = time::precise_time_ns();
    let mut x:f32 = 0.0;
    let mut y:f32 = 0.0;
    let mut z:f32 = 0.0;
    loop {
    	let nt = time::precise_time_ns();
    	let dt = nt - t;
    	t = nt;

    	// update position
    	//z += (dt as f32/16000000 as f32) / 5 as f32;

    	// get reference to the frame
    	let mut target = display.draw();

    	// clear the background
    	target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0),1.0);

    	// draw the terrain
    	let mvp_matrix = perspective_matrix;
    	//let mut mvp_matrix = Into::<[[f32; 4]; 4]>::into(perspective_matrix);
    	terrain.draw(&mut target,&params,&mvp_matrix);

    	// finish drawing and destroy frame object
    	target.finish().unwrap();

    	// render loop
	    for ev in display.poll_events(){
	    	match ev {
	    		glium::glutin::Event::Closed => return,
	    		glium::glutin::Event::Resized(width, height) => {
	    			// update the perspective_matrix
	    			perspective_matrix = cgmath::perspective(cgmath::deg(45.0),width as f32/height as f32,0.0001,100.0);
	    		},
	    		_ => ()
	    	}
	    }
    }
}