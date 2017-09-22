use camera::Camera;

use geometry::{cube_dir, Dir};

use piston::input::keyboard::Key;

use std::collections::HashSet;
use std::f64::consts::FRAC_PI_3;


pub struct Controls {
    pressed_keys: HashSet<Key>,
}


impl Controls {
    pub fn new() -> Self {
        Controls {
            pressed_keys: HashSet::with_capacity(8),
        }
    }

    pub fn press(&mut self, key: Key, cam: &mut Camera) {
        if !self.pressed_keys.contains(&key) {
            self.pressed_keys.insert(key);

            match key {
                Key::W => {
                    let target_angle = cam.target_angle();
                    let turns = (target_angle.radians() / FRAC_PI_3)
                        .round() as usize % 6;

                    let target_pos = cam.target_pos().clone();
                    let target_dir = cube_dir(match turns {
                        0 => Dir::Up,
                        1 => Dir::UpLeft,
                        2 => Dir::DownLeft,
                        3 => Dir::Down,
                        4 => Dir::DownRight,
                        5 => Dir::UpRight,
                        t => panic!("turns = {}", t),
                    });
                    let new_target_pos = target_pos + target_dir;

                    cam.set_target_pos(new_target_pos);
                },
                Key::S => {
                    let target_angle = cam.target_angle();
                    let turns = (target_angle.radians() / FRAC_PI_3)
                        .round() as usize % 6;

                    let target_pos = cam.target_pos().clone();
                    let target_dir = cube_dir(match turns {
                        3 => Dir::Up,
                        4 => Dir::UpLeft,
                        5 => Dir::DownLeft,
                        0 => Dir::Down,
                        1 => Dir::DownRight,
                        2 => Dir::UpRight,
                        t => panic!("turns = {}", t),
                    });
                    let new_target_pos = target_pos + target_dir;

                    cam.set_target_pos(new_target_pos);
                },
                Key::A => cam.inc_target_angle(FRAC_PI_3.into()),
                Key::D => cam.dec_target_angle(FRAC_PI_3.into()),
                _      => (),
            }
        }
    }

    pub fn release(&mut self, key: Key) {
        if self.pressed_keys.contains(&key) {
            self.pressed_keys.remove(&key);
        }
    }
}
