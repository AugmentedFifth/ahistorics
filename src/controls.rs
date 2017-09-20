use camera::Camera;

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
                    let turns =
                        (target_angle / FRAC_PI_3).round() as usize % 6;

                    let &[old_x, old_y] = cam.target_pos();
                    let new_target_pos = match turns {
                        0 => [old_x + old_y         % 2.0, old_y - 1.0],
                        1 => [old_x - (old_y + 1.0) % 2.0, old_y - 1.0],
                        2 => [old_x - 1.0,                 old_y      ],
                        3 => [old_x - (old_y + 1.0) % 2.0, old_y + 1.0],
                        4 => [old_x + old_y         % 2.0, old_y + 1.0],
                        5 => [old_x + 1.0,                 old_y      ],
                        t => panic!("turns = {}", t),
                    };

                    cam.set_target_pos(new_target_pos);
                },
                Key::S => {
                    let target_angle = cam.target_angle();
                    let turns =
                        (target_angle / FRAC_PI_3).round() as usize % 6;

                    let &[old_x, old_y] = cam.target_pos();
                    let new_target_pos = match turns {
                        3 => [old_x + old_y         % 2.0, old_y - 1.0],
                        4 => [old_x - (old_y + 1.0) % 2.0, old_y - 1.0],
                        5 => [old_x - 1.0,                 old_y      ],
                        0 => [old_x - (old_y + 1.0) % 2.0, old_y + 1.0],
                        1 => [old_x + old_y         % 2.0, old_y + 1.0],
                        2 => [old_x + 1.0,                 old_y      ],
                        t => panic!("turns = {}", t),
                    };

                    cam.set_target_pos(new_target_pos);
                },
                Key::A => {
                    let target_angle = cam.target_angle();
                    cam.set_target_angle(target_angle + FRAC_PI_3);
                },
                Key::D => {
                    let target_angle = cam.target_angle();
                    cam.set_target_angle(target_angle - FRAC_PI_3);
                },
                _ => (),
            }
        }
    }

    pub fn release(&mut self, key: Key) {
        if self.pressed_keys.contains(&key) {
            self.pressed_keys.remove(&key);
        }
    }
}
