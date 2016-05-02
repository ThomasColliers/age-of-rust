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
use draw::display_object::{Drawable};
use cgmath::{Matrix4,Point3,Vector3};

fn main() {
	use glium::{DisplayBuild, Surface};
	// can not do multisampling due to glutin bug with_multisampling(16)
	let display = glium::glutin::WindowBuilder::new().with_vsync().with_multisampling(16).with_depth_buffer(24).build_glium().unwrap();

	// window reference
	let window = display.get_window().unwrap();

	// get window size
	let mut display_size = window.get_inner_size_pixels().unwrap();

	// hide the cursor
	window.set_cursor(glium::glutin::MouseCursor::NoneCursor);

	// grab cursor
	// TODO: cursor grabbing doesn't give me access to mouse events, so disabled for now and recentering the cursor instead
	//window.set_cursor_state(glium::glutin::CursorState::Grab).ok().expect("Could not grab mouse cursor.");
	window.set_cursor_position(display_size.0 as i32/2,display_size.1 as i32/2);

	// draw parametres
	let params = glium::DrawParameters {
		depth: glium::Depth {
			test:glium::DepthTest::IfLess,
			write:true,
			.. Default::default()
		},
		.. Default::default()
	};

	// perspective_matrix
	let mut perspective_matrix:Matrix4<f32> = cgmath::perspective(cgmath::deg(45.0),display_size.0 as f32/display_size.1 as f32,0.0001,100.0);

	// view matrix
	let mut camera_position:Point3<f32> = Point3::new(0.0, 15.0, 0.0);
	let camera_target:Point3<f32> = Point3::new(0.0, 0.0, 0.0);
	let camera_up:Vector3<f32> = Vector3::unit_y();

	// setup the shaders
	let mut shader_manager = ShaderManager::new();

	// create terrain
	let terrain = Terrain::new(&display,&mut shader_manager,5);

    // listen for events produced in the window and wait to be received
    let mut old_time:u64 = time::precise_time_ns();
    let mut starting_time:u64 = old_time;
    // render loop
    loop {
    	let new_time = time::precise_time_ns();
    	let delta_time = new_time - old_time;
    	old_time = new_time;
    	let total_time = new_time - starting_time;

    	// update position
    	let seconds = total_time as f32/1600000000.0;
    	camera_position.x = seconds.sin() * 25.0;
    	camera_position.z = seconds.cos() * 25.0;
    	let view_matrix:Matrix4<f32> = cgmath::Matrix4::look_at(camera_position, camera_target, camera_up);

    	// get reference to the frame
    	let mut target = display.draw();

    	// clear the background
    	target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0),1.0);

    	// draw the terrain
    	let mvp_matrix = perspective_matrix * view_matrix;
    	terrain.draw(&mut target,&params,&mvp_matrix);

    	// finish drawing and destroy frame object
    	target.finish().unwrap();

    	// event loop
	    for ev in display.poll_events(){
	    	println!("{:?}", ev);
	    	match ev {
	    		glium::glutin::Event::Closed => return,
	    		glium::glutin::Event::Focused(focus) => {
	    			if focus {
	    				// TODO: regrabbing doesn't seems to work
	    				window.set_cursor_state(glium::glutin::CursorState::Grab).ok().expect("Could not grab mouse cursor.");
	    			}
	    		}
	    		glium::glutin::Event::Resized(_, _) => {
	    			display_size = window.get_inner_size_pixels().unwrap();
	    			perspective_matrix = cgmath::perspective(cgmath::deg(45.0),display_size.0 as f32/display_size.1 as f32,0.0001,100.0);
	    		},
	    		a @ glium::glutin::Event::MouseInput(_, _) => {
                	println!("{:?}", a);
            	},
	    		_ => ()
	    	}
	    }

	    // reset the cursor position
	    window.set_cursor_position(display_size.0 as i32/2,display_size.1 as i32/2);
    }
}