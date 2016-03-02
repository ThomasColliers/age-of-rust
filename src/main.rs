#[macro_use]
extern crate glium;

fn main() {
	use glium::DisplayBuild;
	let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    // listen for evnets produced in the window and wait to be received
    loop {
	    for ev in display.poll_events(){
	    	match ev {
	    		glium::glutin::Event::Closed => return,
	    		_ => ()
	    	}
	    }
    }
}