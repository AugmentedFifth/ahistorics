use camera::Camera;
use draw::SPACING_FACTOR;
use drawable::Drawable;
use failure::Error;
use geometry::{CubePoint, cube_to_real, HEXAGON_POLY};
use graphics::{Context, math::add, polygon::Polygon, types::Color};
use matrix::{m, rot, scale_uni, trans};
use opengl_graphics::GlGraphics;
use rand::{Rng, os::OsRng};
use window::{
    HALF_WINDOW_HEIGHT,
    HALF_WINDOW_WIDTH,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Hex {
    Blank,
    Tile(i32),
}

#[derive(Clone)]
pub struct MapData {
    row_size: usize,
    data:     Vec<Hex>,
    poly:     Polygon,
}

pub struct MapDataIter<'a> {
    i:        usize,
    data:     &'a Vec<Hex>,
    row_size: usize,
}


impl MapData {
    pub fn new(row_size: usize, data: Vec<Hex>, color: Color) -> Self {
        Self {
            row_size,
            data,
            poly: Polygon::new(color),
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

    pub fn iter(&self) -> MapDataIter {
        MapDataIter {
            i: 0,
            data: &self.data,
            row_size: self.row_size,
        }
    }
}

impl<'a> Iterator for MapDataIter<'a> {
    type Item = (&'a Hex, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(hex) = self.data.get(self.i) {
            let y = self.i / self.row_size;
            let x = self.i % self.row_size;

            self.i += 1;

            Some((hex, x, y))
        } else {
            None
        }
    }
}

impl Drawable for MapData {
    fn draw(&self, camera: &Camera, ctx: &Context, gl: &mut GlGraphics) {
        // TODO: `hex_scaled_height` should be dynamic state.
        let hex_scaled_height = 12.0;
        let scale_factor = f64::from(WINDOW_HEIGHT) / hex_scaled_height;

        for (hex, x, y) in self.iter() {
            if hex == &Hex::Blank {
                continue;
            }

            let q = x as i32;
            let r = y as i32 - q / 2;
            let abs_cube_pos = CubePoint::from_q_r(q, r).cast();

            let tile_minus_cam = abs_cube_pos - *camera.pos.pos();

            let cam_rotation = rot(camera.pos.angle().radians());
            let pos = add(
                cam_rotation.vec_mul(cube_to_real(
                    tile_minus_cam,
                    scale_factor,
                )),
                [HALF_WINDOW_WIDTH, HALF_WINDOW_HEIGHT],
            );

            if pos[0] > -scale_factor                          &&
               pos[0] < f64::from(WINDOW_WIDTH) + scale_factor &&
               pos[1] > -scale_factor                          &&
               pos[1] < f64::from(WINDOW_HEIGHT) + scale_factor
            {
                let depth_factor = if let Hex::Tile(depth) = *hex {
                    1.0 + f64::from(depth) / 16.0
                } else {
                    1.0
                };

                let transform =
                    cam_rotation *
                    scale_uni(
                        scale_factor *
                        (SPACING_FACTOR * depth_factor).min(0.975)
                    ) *
                    trans(pos) *
                    m(ctx.transform);

                self.poly.draw(
                    HEXAGON_POLY,
                    &ctx.draw_state,
                    transform.repr,
                    gl
                );
            }
        }
    }
}

pub fn simulated_map_data(side_len: usize,
                          color:    Color) -> Result<MapData, Error>
{
    let area = side_len * side_len;
    let mut data = Vec::with_capacity(area);

    let mut rng = OsRng::new()?;
    for _ in 0..area {
        data.place_back() <- if rng.gen() {
            Hex::Blank
        } else {
            Hex::Tile(rng.gen_range(-6, 3))
        };
    }

    Ok(MapData::new(side_len, data, color))
}
