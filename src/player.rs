use geometry::{cube_dir, CubePoint, Dir};

use positioned::Positioned;

use std::f64::consts::FRAC_PI_3;

use transitioned_grid_pos::TransitionedGridPos;


#[derive(Debug)]
pub struct Player {
    /// Position of player in terms of the underlying cubic coordinate system.
    pub pos: TransitionedGridPos,
}


impl Player {
    pub fn new(anim_time: f64, start_pos: CubePoint<f64>) -> Self {
        Player {
            pos: TransitionedGridPos::new(anim_time, start_pos),
        }
    }
}

impl Positioned for Player {
    fn unit_move(&mut self, forwards: bool) {
        let target_angle = self.pos.target_angle();
        let turns = (target_angle.radians() / FRAC_PI_3)
            .round() as usize % 6;

        let target_pos = *self.pos.target_pos();
        let target_dir = cube_dir(match turns {
            0 => Dir::Up,
            1 => Dir::UpLeft,
            2 => Dir::DownLeft,
            3 => Dir::Down,
            4 => Dir::DownRight,
            5 => Dir::UpRight,
            t => panic!("turns == {}", t),
        });
        let new_target_pos = if forwards {
            target_pos + target_dir
        } else {
            target_pos - target_dir
        };

        self.pos.set_target_pos(new_target_pos);
    }
}
