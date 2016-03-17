extern crate glium;

use rand::{thread_rng, Rng};

use math3d::Vertex;
use glium::backend::Facade;
use draw::display_object::DisplayInfo;
use draw::shaders::ShaderManager;

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

pub struct Terrain<'a> {
	data:Vec<Tile>,
	display_info:DisplayInfo<'a>,
}

impl<'a> Terrain<'a> {
	pub fn new<F>(display: &F, shader_manager: &'a ShaderManager, size:u16) -> Terrain<'a> where F: Facade + Clone {
		// load the shader
		let shader = shader_manager.load("identity.vert","identity.frag").unwrap();

		// create the terrain data
		let mut terrain = Terrain {
			data:Vec::with_capacity((size*size) as usize),
			display_info:DisplayInfo { x:0f32, y:0f32, z:0f32, shader:Some(shader) }
		};
		
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
		for n in 0..(size*size) {
			terrain.data.push(Tile { typ:*rng.choose(&choices).unwrap(), height:0f32 });
		}

		//println!("Terrain: {:?}",terrain.data);


		// generate the geometry for the terrain
		terrain.generate_geometry(display,size);

		terrain
	}

	fn generate_geometry<F>(&mut self, display:&F, size:u16) where F: Facade + Clone {
		// create the vertices
		let mut vertices = Vec::new();
		let size_f = size as f32;
		for index in 0..(size*size) {
			let index_f = index as f32;
			let row = index / size;
			let column = index % size;
			// TODO: generate the actual normals
			vertices.push(Vertex { position:[column as f32, row as f32, self.data[index as usize].height], texcoords:[column as f32/size_f,row as f32/size_f], normal:[0f32,0f32,1f32] });
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
	}

	pub fn draw(&mut self){

	}
}