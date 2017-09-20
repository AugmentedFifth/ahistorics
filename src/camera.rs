use geometry::{bezier2, grid_pos_to_real, modulo};

use graphics::math::{add, mul_scalar, sub, Vec2d};

use std::f64::consts::PI;


pub struct Camera {
    /// Width of camera's view, in terms of the outer radius of a hexagon.
    width:        f64,
    /// Height of camera's view, in terms of the outer radius of a hexagon.
    height:       f64,
    /// Total time that an animation phase of the camera takes.
    anim_time:    f64,
    /// Position of camera in terms of the underlying Cartesian grid of the
    /// hexagons.
    pos:          Vec2d,
    /// Position that the camera is moving towards. Should be set to the
    /// current player position, or just wherever the camera should be
    /// focussing.
    target_pos:   Vec2d,
    /// Previous position that the camera was at, only applicable when the
    /// camera is animating.
    prev_pos:     Vec2d,
    /// Current progress of transition from `pos` to `target_pos`. `<= 0` is
    /// "just started", `>= 1` is "complete, no animation in progress".
    pos_state:    f64,
    /// Angle the camera is oriented at, in radians.
    angle:        f64,
    /// Angle that the camera is rotating to. Should be set to the orientation
    /// that the camera should currently be on.
    ///
    /// `0.0 <= target_angle < 2.0 * PI`
    target_angle: f64,
    /// Previous angle that the camera was at, only applicable when the camera
    /// is animating.
    prev_angle:   f64,
    /// Current progress of transition from `angle` to `target_angle`. `<= 0`
    /// is "just started", `>= 1` is "complete, no animation in progress".
    angle_state:  f64,
}


impl Camera {
    pub fn new(
        width:     f64,
        height:    f64,
        anim_time: f64,
        start_pos: Vec2d
    ) -> Self {
        Camera {
            width,
            height,
            anim_time,
            pos:          start_pos,
            target_pos:   start_pos,
            prev_pos:     start_pos,
            pos_state:    1.0,
            angle:        0.0,
            target_angle: 0.0,
            prev_angle:   0.0,
            angle_state:  1.0,
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

    pub fn target_pos(&self) -> &Vec2d {
        &self.target_pos
    }

    pub fn target_angle(&self) -> f64 {
        self.target_angle
    }

    /// Testing purposes only.
    pub fn inc_angle(&mut self, inc: f64) {
        self.angle += inc;
    }

    pub fn set_target_pos(&mut self, target: Vec2d) {
        self.pos_state = 0.0;
        self.prev_pos = self.pos;
        self.target_pos = target;
    }

    pub fn set_target_angle(&mut self, target: f64) {
        self.angle_state = 0.0;
        self.prev_angle = self.angle;
        self.target_angle = modulo(target, 2.0 * PI);
    }

    pub fn step(&mut self, dt: f64) {
        if self.pos != self.target_pos {
            if self.pos_state >= 1.0 {
                self.pos_state = dt / self.anim_time;

                self.pos = self.target_pos;
                self.prev_pos = self.pos;
            } else {
                self.pos_state += dt / self.anim_time;

                let new_pos_progress = bezier2(
                    0.0,
                    0.75,
                    1.0,
                    self.pos_state.min(1.0)
                );
                let disp = mul_scalar(
                    sub(self.target_pos, self.prev_pos),
                    new_pos_progress
                );

                self.pos = add(self.prev_pos, disp);
            }
        }

        if self.angle != self.target_angle {
            if self.angle_state >= 1.0 {
                self.angle_state = dt / self.anim_time;

                self.angle = self.target_angle;
                self.prev_angle = self.angle;
            } else {
                self.angle_state += dt / self.anim_time;

                let new_angle_progress = bezier2(
                    0.0,
                    0.75,
                    1.0,
                    self.angle_state.min(1.0)
                );
                let disp =
                    (self.target_angle - self.prev_angle) * new_angle_progress;

                self.angle = self.prev_angle + disp;
            }
        }
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
