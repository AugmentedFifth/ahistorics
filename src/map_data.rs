use rand::Rng;
use rand::os::OsRng;


#[derive(Debug, PartialEq, Eq)]
pub enum Hex {
    Blank,
    Tile,
}

pub struct MapData {
    row_size: usize,
    data:     Vec<Hex>,
}


impl MapData {
    pub fn new(row_size: usize, data: Vec<Hex>) -> Self {
        MapData {
            row_size: row_size,
            data:     data,
        }
    }

    pub fn data(&self) -> &Vec<Hex> {
        &self.data
    }

    pub fn row_size(&self) -> usize {
        self.row_size
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Hex> {
        self.data.get(y * self.row_size + x)
    }

    pub fn for_each<F: FnMut(&Hex, usize, usize) -> ()>(&self, mut f: F) {
        for (y, chunk) in self.data.chunks(self.row_size).enumerate() {
            for (x, hex) in chunk.iter().enumerate() {
                f(hex, x, y);
            }
        }
    }
}

pub fn simulated_map_data(side_len: usize) -> MapData {
    let area = side_len * side_len;
    let mut data = Vec::with_capacity(area);

    let mut rng = OsRng::new().expect(
        "Failed to initialize operating-system-based RNG."
    );
    for _ in 0..area {
        data.place_back() <- if rng.gen() { Hex::Blank } else { Hex::Tile };
    }

    MapData {
        row_size: side_len,
        data:     data,
    }
}
