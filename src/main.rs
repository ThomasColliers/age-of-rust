#[macro_use]
extern crate glium;

extern crate rand;

mod world;
use world::terrain::Terrain;

mod math3d;

fn main() {
	use glium::{DisplayBuild, Surface};
	// can not do multisampling due to glutin bug with_multisampling(16)
	let display = glium::glutin::WindowBuilder::new().with_vsync().build_glium().unwrap();

	// draw parametres
	let params = glium::DrawParameters {
		/*depth: glium::Depth {
			test:glium::DepthTest::IfLess,
			write:true,
			.. Default::default()
		},*/
		.. Default::default()
	};

	let terrain = Terrain::new(20);

	/*
	// init scene
	// create a triangle
	let vertex1 = Vertex { position: [-0.5,-0.5] };
	let vertex2 = Vertex { position: [0.0,0.5] };
	let vertex3 = Vertex { position: [0.5,-0.25] };
	let shape = vec![vertex1,vertex2,vertex3];
	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	// shader program
	let vertex_shader_src = r#"
		#version 430

		in vec2 position;

		void main(){
			gl_Position = vec4(position, 0.0, 1.0);
		}
	"#;
	let fragment_shader_src = r#"
		#version 430

		out vec4 color;

		void main(){
			color = vec4(1.0, 0.0, 0.0, 1.0);
		}
	"#;
	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();*/

	// generate a terrain


    // listen for events produced in the window and wait to be received
    loop {
    	// get reference to the frame
    	let mut target = display.draw();

    	// clear the background
    	target.clear_color(0.0, 0.0, 0.0, 1.0);

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