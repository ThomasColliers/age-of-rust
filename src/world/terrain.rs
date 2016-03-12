use rand::{thread_rng, Rng};

use math3d::Vertex;

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
	data:Vec<Tile>
}

impl Terrain {
	pub fn new(size:u16) -> Terrain {
		// create the terrain data
		let mut terrain = Terrain {
			data:Vec::with_capacity((size*size) as usize),
		};
		
		// fill up the terrain with random tiles
		let choices = [
			TerrainType::Water,
			TerrainType::Grass,
			TerrainType::TallGrass,
			TerrainType::Sand,
			TerrainType::Snow,
			TerrainType::Tundra
		];
		let mut rng = thread_rng();
		for n in 0..(size*size) {
			terrain.data.push(Tile { typ:*rng.choose(&choices).unwrap(), height:0f32 });
		}

		//println!("Terrain: {:?}",terrain.data);

		// generate the geometry for the terrain
		terrain.generate_geometry(size);

		terrain
	}

	fn generate_geometry(&mut self, size:u16){
		// create the vertices
		let mut vertices = Vec::new();
		let size_f = size as f32;
		for index in 0..(size*size) {
			let index_f = index as f32;
			let row = index / size;
			let column = index % size;
			vertices.push(Vertex { position:[column as f32, row as f32, self.data[index as usize].height], texcoords:[column as f32/size_f,row as f32/size_f], normal:[0f32,0f32,1f32] });
		}
	}

	pub fn draw(&mut self){

	}
}