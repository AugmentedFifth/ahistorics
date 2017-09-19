use camera::Camera;

use piston::input::keyboard::Key;

use std::collections::HashSet;


pub struct Controls {
    pressed_keys: HashSet<Key>,
    camera:       Camera,
}


impl Controls {
    pub fn new(camera: Camera) -> Self {
        Controls {
            pressed_keys: HashSet::with_capacity(8),
            camera
        }
    }

    pub fn press(&mut self, key: Key) {
        if !self.pressed_keys.contains(key) {
            self.pressed_keys.insert(key);

            match key {

            }
        }
    }

    pub fn release(&mut self, key: Key) {
        if self.pressed_keys.contains(key) {
            self.pressed_keys.remove(key);

            match key {

            }
        }
    }
}
