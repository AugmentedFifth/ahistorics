use camera::Camera;
use fnv::FnvHashSet as Set;
use piston::input::keyboard::Key;
use player::Player;
use positioned::Positioned;


pub struct Controls {
    pressed_keys: Set<Key>,
}


impl Controls {
    pub fn new() -> Self {
        Controls {
            pressed_keys: Set::with_capacity_and_hasher(8, Default::default()),
        }
    }

    pub fn press(&mut self, key: Key, cam: &mut Camera, player: &mut Player) {
        if self.pressed_keys.insert(key) {
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
                    cam.turn(true);
                    player.turn(true);
                },
                Key::D => {
                    cam.turn(false);
                    player.turn(false);
                },
                _ => (),
            }
        }
    }

    pub fn release(&mut self, key: &Key) {
        self.pressed_keys.remove(key);
    }
}
