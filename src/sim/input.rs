use std::collections::HashSet;
use winit::event::VirtualKeyCode;

#[derive(Default)]
pub struct InputController {
    pub keys: HashSet<VirtualKeyCode>,
    pub keys_last: HashSet<VirtualKeyCode>,
}

impl InputController {
    pub fn update(&mut self) {
        self.keys_last = self.keys.clone();
    }
    pub fn press(&mut self, key: VirtualKeyCode) {
        self.keys.insert(key);
    }
    pub fn release(&mut self, key: VirtualKeyCode) {
        self.keys.remove(&key);
    }
    pub fn is_one_of_key_active(&self, keys: Vec<VirtualKeyCode>) -> bool {
        for key in keys {
            if self.is_key_active(key) {
                return true;
            }
        }
        false
    }
    pub fn is_key_active(&self, key: VirtualKeyCode) -> bool {
        self.keys.contains(&key)
    }
    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys.contains(&key) && !self.keys_last.contains(&key)
    }
    pub fn is_key_released(&self, key: VirtualKeyCode) -> bool {
        !self.keys.contains(&key) && self.keys_last.contains(&key)
    }
}
