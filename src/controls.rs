use camera::Camera;

use piston::input::keyboard::Key;

use player::Player;

use positioned::Positioned;

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

    pub fn press(&mut self, key: Key, cam: &mut Camera, player: &mut Player) {
        if !self.pressed_keys.contains(&key) {
            self.pressed_keys.insert(key);

            match key {
                Key::W => {
                    cam.unit_move(true);
                    player.unit_move(true);
                },
                Key::S => {
                    cam.unit_move(false);
                    player.unit_move(false);
                },
                Key::A => {
                    cam.pos.inc_target_angle(FRAC_PI_3.into());
                    player.pos.inc_target_angle(FRAC_PI_3.into());
                },
                Key::D => {
                    cam.pos.dec_target_angle(FRAC_PI_3.into());
                    player.pos.dec_target_angle(FRAC_PI_3.into());
                },
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
