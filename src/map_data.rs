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
            row_size,
            data,
        }
    }

    pub fn data(&self) -> &Vec<Hex> {
        &self.data
    }

    pub fn row_size(&self) -> usize {
        self.row_size
    }

    /// Calculates the number of rows represented by this data.
    pub fn rows(&self) -> usize {
        self.data.len() / self.row_size
    }

    /// Alias for `::row_size()`.
    pub fn cols(&self) -> usize {
        self.row_size
    }

    pub fn get_cube(&self, x: i32, _: i32, z: i32) -> Option<&Hex> {
        self.get_axial(x, z)
    }

    pub fn get_axial(&self, q: i32, r: i32) -> Option<&Hex> {
        self.data.get(((r + q / 2) * self.row_size as i32 + q) as usize)
    }

    pub fn get_rect(&self, x: usize, y: usize) -> Option<&Hex> {
        self.data.get(y * self.row_size + x)
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
        data,
    }
}
