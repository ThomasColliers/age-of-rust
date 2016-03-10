#[derive(Debug)]
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
	pub fn new(size:usize, seed: Option<u32>) -> Terrain {
		// create the terrain data
		let mut terrain = Terrain {
			data:Vec::with_capacity(size*size),
		};
		
		// fill up the terrain with random tiles
		for n in 0..(size*size) {
			terrain.data.push(Tile { typ:TerrainType::Water });
		}

		println!("Terrain: {:?}",terrain.data);

		terrain
	}
}