use std::collections::HashSet;

#[derive(Debug)]
pub struct KeyboardHandler {
    pub key_pool: HashSet<u32>,
}

impl KeyboardHandler {
    pub fn process_press(&mut self, input: u32) {
        self.key_pool.insert(input);
    }

    pub fn process_release(&mut self, input: u32) {
        self.key_pool.remove(&input);
    }

    pub fn process_with<T>(&self, mut processor: T)
    where
        T: FnMut(u32),
    {
        for key in &self.key_pool {
            processor(*key);
        }
    }

    pub fn new() -> KeyboardHandler {
        KeyboardHandler {
            key_pool: HashSet::new(),
        }
    }
}
