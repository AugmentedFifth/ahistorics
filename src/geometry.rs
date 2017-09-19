use graphics::math::Vec2d;
use graphics::types::Polygon;


pub const SQRT_3_ON_2: f64 =
    0.8660254037844385965883020617184229195117950439453125;

pub const HEXAGON_POLY: Polygon<'static> = &[
    [ 0.0,          1.0],
    [ SQRT_3_ON_2,  0.5],
    [ SQRT_3_ON_2, -0.5],
    [ 0.0,         -1.0],
    [-SQRT_3_ON_2, -0.5],
    [-SQRT_3_ON_2,  0.5],
];


pub fn grid_pos_to_real(grid_pos: &Vec2d) -> Vec2d {
    let x = grid_pos[0];
    let y = grid_pos[1];
    let sq_wave = ((y + 1.0) % 2.0 - 1.0).abs();

    [(2.0 * x + sq_wave) * SQRT_3_ON_2, y * 1.5]
}

pub fn real_pos_to_grid(real_pos: &Vec2d, round_dir: i8) -> Vec2d {
    let x = real_pos[0];
    let y = real_pos[1];
    let sq_wave = ((y / 1.5 + 1.0) % 2.0 - 1.0).abs();

    let real_x = (x / SQRT_3_ON_2 - sq_wave) / 2.0;
    let real_y = y / 1.5;

    if round_dir > 0 {
        [real_x.ceil(), real_y.ceil()]
    } else if round_dir == 0 {
        [real_x.round(), real_y.round()]
    } else {
        [real_x.floor(), real_y.floor()]
    }
}
