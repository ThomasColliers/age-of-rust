use rand::{thread_rng, Rng};

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
}

pub struct Terrain {
	data:Vec<Tile>,
}

impl Terrain {
	pub fn new(size:usize) -> Terrain {
		// create the terrain data
		let mut terrain = Terrain {
			data:Vec::with_capacity(size*size),
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
			terrain.data.push(Tile { typ:*rng.choose(&choices).unwrap() });
		}

		println!("Terrain: {:?}",terrain.data);

		terrain
	}
}