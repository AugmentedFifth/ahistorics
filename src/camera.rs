use geometry::{Angle, cube_dir, CubePoint, Dir};
use positioned::Positioned;
use std::f64::consts::FRAC_PI_3;
use temporal::Temporal;
use transitioned_grid_pos::TransitionedGridPos;


pub struct Camera {
    pos: TransitionedGridPos,
}


impl Camera {
    pub fn new(anim_time: f64, start_pos: CubePoint<f64>) -> Self {
        Camera {
            pos: TransitionedGridPos::new(anim_time, start_pos),
        }
    }
}

impl Positioned for Camera {
    fn unit_move(&mut self, forwards: bool) {
        let target_angle = self.pos.target_angle();
        let turns = (target_angle.radians() / FRAC_PI_3)
            .round() as u8 % 6;

        let target_pos = *self.pos.target_pos();
        let target_dir = cube_dir(match turns {
            0 => Dir::Up,
            1 => Dir::UpLeft,
            2 => Dir::DownLeft,
            3 => Dir::Down,
            4 => Dir::DownRight,
            5 => Dir::UpRight,
            _ => unreachable!(),
        });
        let new_target_pos = if forwards {
            target_pos + target_dir
        } else {
            target_pos - target_dir
        };

        self.pos.set_target_pos(new_target_pos);
    }

    fn turn(&mut self, anticlockwise: bool) {
        if anticlockwise {
            self.pos.inc_target_angle(FRAC_PI_3);
        } else {
            self.pos.dec_target_angle(FRAC_PI_3);
        }
    }

    fn pos(&self) -> &CubePoint<f64> {
        self.pos.pos()
    }

    fn angle(&self) -> Angle {
        self.pos.angle()
    }
}

impl Temporal for Camera {
    fn step(&mut self, dt: f64) {
        self.pos.step(dt);
    }
}
