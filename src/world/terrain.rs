extern crate glium;

use rand::{thread_rng, Rng};
use std::rc::Rc;

use draw::Vertex;
use glium::backend::Facade;
use draw::display_object::{Frame,Drawable,HasFrame};
use draw::shaders::ShaderManager;
use glium::program::Program;
use glium::{VertexBuffer,IndexBuffer,Surface};
use na::{Mat4};
use glium::uniforms::AsUniformValue;

#[derive(Debug, Copy, Clone)]
pub enum TerrainType {
	Water,
	Grass,
	TallGrass,
	Sand,
	Snow,
	Tundra
}

#[derive(Debug)]
pub struct Tile {
	typ:TerrainType,
	height:f32,
}

pub struct Terrain {
	data:Vec<Tile>,
	frame:Frame<f32>,
	shader:Rc<Program>,
	vertex_buffer:VertexBuffer<Vertex>,
	index_buffer:IndexBuffer<u16>,
}

impl Terrain {
	pub fn new<F>(display: &F, shader_manager: &mut ShaderManager, size:u16) -> Terrain where F: Facade + Clone {
		// load the shader
		let shader = shader_manager.load(display,"identity.vert","identity.frag").unwrap();

		// fill up the terrain with random tiles
		let choices = [
			TerrainType::Water,
			TerrainType::Grass,
			TerrainType::TallGrass,
			TerrainType::Sand,
			TerrainType::Snow,
			TerrainType::Tundra,
		];
		let mut rng = thread_rng();
		let mut terrain_data = Vec::with_capacity((size*size) as usize);
		for n in 0..(size*size) {
			terrain_data.push(Tile { typ:*rng.choose(&choices).unwrap(), height:0f32 });
		}

		// generate the geometry

		// create the vertices
		let mut vertices = Vec::new();
		let size_f = size as f32;
		for index in 0..(size*size) {
			let index_f = index as f32;
			let row = index / size;
			let column = index % size;
			// TODO: generate the actual normals
			vertices.push(Vertex { position:[column as f32, row as f32, terrain_data[index as usize].height], texcoords:[column as f32/size_f,row as f32/size_f], normal:[0f32,0f32,1f32] });
		}

		// create the vertex buffer
		let vertex_buffer = glium::VertexBuffer::new(display,&vertices).unwrap();

		// create the indices
		let mut indices = Vec::new();
		let columnsize = size - 1;
		for index in 0..(columnsize*columnsize){
			let row = index / columnsize;
			let column = index % columnsize;
			// build the 2 triangles for this square
			let baseindex = row*size+column;
			indices.push(baseindex as u16);
			indices.push(baseindex+size as u16);
			indices.push(baseindex+size+1 as u16 );
			indices.push(baseindex as u16);
			indices.push(baseindex+size+1 as u16);
			indices.push(baseindex+1 as u16 );
		}

		// create the index buffer
		let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

		// create the terrain data
		let mut terrain = Terrain {
			data:terrain_data,
			frame:Frame::<f32>::new(),
			shader:shader,
			vertex_buffer:vertex_buffer,
			index_buffer:index_buffer,
		};

		println!("{:?}", terrain.frame);

		terrain
	}
}

impl Drawable for Terrain {
	fn draw(&self, target:&mut glium::Frame, params:&glium::DrawParameters, mvp_matrix:&Mat4<f32>) {
		/*Into::<[[f32; 4]; 4]>::into(matrix)*/
		target.draw(&self.vertex_buffer,&self.index_buffer,self.shader.as_ref(),&uniform! { mvpMatrix:*mvp_matrix.as_ref() },params);
		//target.draw(&vertex_buffer,&indices,&program,&glium::uniforms::EmptyUniforms,&params).unwrap();
	}
}

impl HasFrame<f32> for Terrain {
	fn get_frame(&self) -> &Frame<f32> {
		&self.frame
	}
}