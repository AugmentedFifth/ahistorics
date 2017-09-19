use geometry::grid_pos_to_real;

use graphics::math::Vec2d;


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

        let half_width  = self.width  / 2.0;
        let half_height = self.height / 2.0;

        let radius =
            (half_width * half_width + half_height * half_height).sqrt();

        let max_y = real_pos[1] + radius;
        let min_y = (real_pos[1] - radius).max(0.0);

        let max_x = real_pos[0] + radius;
        let min_x = (real_pos[0] - radius).max(0.0);

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

        for x in min_col .. max_col {
            for y in min_row .. max_row {
                draw_fn(x, y);
            }
        }
    }
}
