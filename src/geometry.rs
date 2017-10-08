use graphics::math::Vec2d;
use graphics::types::Polygon;

use std::f64::consts::PI;

use std::ops::{Add, AddAssign, Sub, SubAssign, Neg};


pub const PI_2: f64 = 2.0 * PI;

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

const CUBE_DIRS: &'static [CubePoint<i32>; 6] = &[
    CubePoint { a:  1, b: -1, c:  0 },
    CubePoint { a:  1, b:  0, c: -1 },
    CubePoint { a:  0, b:  1, c: -1 },
    CubePoint { a: -1, b:  1, c:  0 },
    CubePoint { a: -1, b:  0, c:  1 },
    CubePoint { a:  0, b: -1, c:  1 },
];


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    DownRight = 0,
    UpRight   = 1,
    Up        = 2,
    UpLeft    = 3,
    DownLeft  = 4,
    Down      = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CubePoint<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AxialPoint {
    pub q: i32,
    pub r: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Angle {
    radians: f64,
}


impl<T: Clone + Neg<Output=T> + Sub<Output=T>> CubePoint<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        CubePoint { a, b, c }
    }

    pub fn from_q_r(q: T, r: T) -> Self {
        let b = -q.clone() - r.clone();

        CubePoint { a: q, b, c: r }
    }

    pub fn map<U, F: Fn(T) -> U>(self, map_fn: F) -> CubePoint<U> {
        CubePoint {
            a: map_fn(self.a),
            b: map_fn(self.b),
            c: map_fn(self.c),
        }
    }

    pub fn cast<U: From<T>>(self) -> CubePoint<U> {
        CubePoint {
            a: self.a.into(),
            b: self.b.into(),
            c: self.c.into(),
        }
    }
}

impl<T: From<i32>> From<AxialPoint> for CubePoint<T> {
    fn from(axial: AxialPoint) -> Self {
        let a = axial.q.into();
        let c = axial.r.into();
        let b = (-axial.q - axial.r).into();

        CubePoint { a, b, c }
    }
}

impl<T: Into<i32>> Into<AxialPoint> for CubePoint<T> {
    fn into(self) -> AxialPoint {
        AxialPoint {
            q: self.a.into(),
            r: self.c.into(),
        }
    }
}

impl<U, T: Add<Output=U>> Add for CubePoint<T> {
    type Output = CubePoint<U>;

    fn add(self, rhs: Self) -> Self::Output {
        CubePoint {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
        }
    }
}

impl<T: AddAssign> AddAssign for CubePoint<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.b += rhs.b;
        self.c += rhs.c;
    }
}

impl<T: Sub<Output=T>> Sub for CubePoint<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        CubePoint {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
            c: self.c - rhs.c,
        }
    }
}

impl<T: SubAssign> SubAssign for CubePoint<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.a -= rhs.a;
        self.b -= rhs.b;
        self.c -= rhs.c;
    }
}

impl AxialPoint {
    pub fn new(q: i32, r: i32) -> Self {
        AxialPoint { q, r }
    }
}

impl Add for AxialPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        AxialPoint {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl AddAssign for AxialPoint {
    fn add_assign(&mut self, rhs: Self) {
        self.q += rhs.q;
        self.r += rhs.r;
    }
}

impl Angle {
    pub fn new(radians: f64) -> Self {
        Angle { radians: modulo(radians, PI_2) }
    }

    pub fn radians(&self) -> f64 {
        self.radians
    }

    pub fn lerp(&self, end: &Self, t: f64) -> Self {
        let diff = end.radians - self.radians;

        if diff > PI {
            Angle {
                radians: modulo(
                    (1.0 - t) * (self.radians + PI_2) + t * end.radians,
                    PI_2
                )
            }
        } else if diff < -PI {
            Angle {
                radians: modulo(
                    (1.0 - t) * self.radians + t * (end.radians + PI_2),
                    PI_2
                )
            }
        } else {
            Angle { radians: (1.0 - t) * self.radians + t * end.radians }
        }
    }
}

impl From<f64> for Angle {
    fn from(radians: f64) -> Self {
        Angle::new(radians)
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Angle) -> bool {
        self.radians == other.radians
    }
}

impl Eq for Angle {}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians + rhs.radians)
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.radians = modulo(self.radians + rhs.radians, PI_2);
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians - rhs.radians)
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.radians = modulo(self.radians - rhs.radians, PI_2);
    }
}


pub fn cube_dir(dir: Dir) -> CubePoint<i32> {
    CUBE_DIRS[dir as usize]
}

pub fn cube_to_real<T: Into<f64>>(cube_pos: CubePoint<T>, size: f64) -> Vec2d {
    let cube_a = cube_pos.a.into();
    let x = size * 1.5 * cube_a;
    let y = size * SQRT_3 * (cube_pos.c.into() + cube_a as f64 / 2.0);

    [x, y]
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

pub fn lerp(v0: f64, v1: f64, t: f64) -> f64 {
    (1.0 - t) * v0 + t * v1
}

pub fn cube_lerp<T: Into<f64>, U: Into<f64>>(
    v0: CubePoint<T>,
    v1: CubePoint<U>,
    t:  f64
) -> CubePoint<f64> {
    CubePoint {
        a: lerp(v0.a.into(), v1.a.into(), t),
        b: lerp(v0.b.into(), v1.b.into(), t),
        c: lerp(v0.c.into(), v1.c.into(), t),
    }
}
