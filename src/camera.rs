use geometry::{grid_pos_to_real, SQRT_3_ON_2};

use graphics::math::{add, Vec2d};

use matrix::rot;


pub struct Camera {
    /// Width of camera's view, in terms of the outer radius of a hexagon.
    width:  f64,
    /// Height of camera's view, in terms of the outer radius of a hexagon.
    height: f64,
    /// Position of camera in terms of the underlying Cartesian grid of the
    /// hexagons.
    pos:    Vec2d,
    /// Angle the camera is oriented at, in radians.
    angle:  f64,
}


impl Camera {
    pub fn new(width: f64, height: f64, start_pos: Vec2d) -> Self {
        Camera {
            width:  width,
            height: height,
            pos:    start_pos,
            angle:  0.0,
        }
    }

    pub fn pos(&self) -> &Vec2d {
        &self.pos
    }

    pub fn x(&self) -> f64 {
        self.pos[0]
    }

    pub fn y(&self) -> f64 {
        self.pos[1]
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Testing purposes only.
    pub fn inc_angle(&mut self, inc: f64) {
        self.angle += inc;
    }

    pub fn draw<F: FnMut(usize, usize) -> ()>(
        &self,
        mut draw_fn: F,
        cols:        usize,
        rows:        usize
    ) {
        let real_pos = grid_pos_to_real(&self.pos);

        let rotation = rot(self.angle);
        let half_width  = self.width  / 2.0;
        let half_height = self.height / 2.0;

        //let radius =
            //(half_width * half_width + half_height * half_height).sqrt();

        let tr = add(real_pos, rotation.vec_mul([ half_height,  half_width]));
        let tl = add(real_pos, rotation.vec_mul([ half_height, -half_width]));
        let bl = add(real_pos, rotation.vec_mul([-half_height, -half_width]));
        let br = add(real_pos, rotation.vec_mul([-half_height,  half_width]));

        let max_y = tr[1].max(tl[1]).max(bl[1]).max(br[1]);
        let sq_wave_max = ((max_y / 1.5 + 1.0) % 2.0 - 1.0).abs();
        let min_y = tr[1].min(tl[1]).min(bl[1]).min(br[1]);
        let sq_wave_min = ((min_y / 1.5 + 1.0) % 2.0 - 1.0).abs();

        let max_x =
            (tr[0].max(tl[0]).max(bl[0]).max(br[0]) / SQRT_3_ON_2 -
                sq_wave_max.min(sq_wave_min)) / 2.0;
        let min_x =
            (tr[0].min(tl[0]).min(bl[0]).min(br[0]) / SQRT_3_ON_2 -
                sq_wave_max.max(sq_wave_min)) / 2.0;

        let min_col = min_x.floor().max(0.0) as usize;
        let max_col = max_x.ceil();
        let max_col = if max_col >= 0.0 {
            (max_col as usize).min(cols)
        } else {
            panic!("max_col < 0");
        };

        let min_row = min_y.floor().max(0.0) as usize;
        let max_row = max_y.ceil();
        let max_row = if max_row >= 0.0 {
            (max_row as usize).min(rows)
        } else {
            panic!("max_row < 0");
        };

        for x in min_col .. max_col + 1 {
            for y in min_row .. max_row + 1 {
                draw_fn(x, y);
            }
        }
    }
}
