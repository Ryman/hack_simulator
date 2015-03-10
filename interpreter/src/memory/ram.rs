use std::ops::{Deref, DerefMut};
use memory::{RAM_SIZE, Word};

pub struct Ram(Vec<Word>);

impl Ram {
    pub fn new() -> Ram { Ram(vec![0; RAM_SIZE]) }
}

// Implement both Deref and DerefMut for Ram
impl Deref for Ram {
    type Target = [Word];

    fn deref<'a>(&'a self) -> &'a [Word] {
        &self.0
    }
}

impl DerefMut for Ram {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [Word] {
        &mut self.0
    }
}
