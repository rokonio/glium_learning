use std::collections::HashSet;

pub struct KeyboardHandler {
    pub key_pool: HashSet<u32>,
}

impl KeyboardHandler {
    pub fn process_input(&mut self, input: u32) {
        if self.key_pool.contains(&input) {
            self.key_pool.remove(&input);
        } else {
            self.key_pool.insert(input);
        }
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
