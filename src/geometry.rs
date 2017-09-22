use graphics::math::Vec2d;
use graphics::types::Polygon;


pub const SQRT_3: f64 = 1.732050807568877193176604123436845839023590087890625;

pub const SQRT_3_ON_2: f64 =
    0.8660254037844385965883020617184229195117950439453125;

pub const HEXAGON_POLY: Polygon<'static> = &[
    [ 1.0,          0.0],
    [ 0.5,  SQRT_3_ON_2],
    [-0.5,  SQRT_3_ON_2],
    [-1.0,          0.0],
    [-0.5, -SQRT_3_ON_2],
    [ 0.5, -SQRT_3_ON_2],
];

#[derive(Copy, PartialEq, Eq)]
pub struct AxialPoint {
    pub q: i32,
    pub r: i32,
}


impl AxialPoint {
    pub fn new(q: i32, r: i32) -> Self {
        AxialPoint { q, r }
    }

    pub fn as_vec2d(&self) -> Vec2d {
        [self.q as f64, self.r as f64]
    }

    pub fn as_arr(&self) -> [i32; 2] {
        [self.q, self.r]
    }
}

impl Clone for AxialPoint {
    fn clone(&self) -> Self {
        *self
    }
}

pub fn axial_to_real(axial_pos: AxialPoint, size: f64) -> Vec2d {
    let x = size * 1.5 * axial_pos.q as f64;
    let y = size * SQRT_3 * (axial_pos.r as f64 + axial_pos.q as f64 / 2.0);

    [x, y]
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

/// Calculates one point in a one-dimensional quadratic Bezier curve.
///
/// # Arguments
///
/// * `p0` - First control point (start point).
/// * `p1` - Second control point (determines curvature).
/// * `p2` - Third control point (end point).
/// * `t`  - Time, where `0 <= t <= 1`.
pub fn bezier2(p0: f64, p1: f64, p2: f64, t: f64) -> f64 {
    let time_complement = 1.0 - t;

    time_complement * (time_complement * p0 + t * p1) +
    t               * (time_complement * p1 + t * p2)
}

pub fn modulo(a: f64, b: f64) -> f64 {
    if a >= 0.0 {
        a % b
    } else {
        (b + a % b) % b
    }
}
