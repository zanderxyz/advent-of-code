use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Increment<T> {
    fn increment(&mut self, key: T, count: usize);
    fn decrement(&mut self, key: T, count: usize);
    fn decrement_delete(&mut self, key: T, count: usize);
}

impl<T> Increment<T> for HashMap<T, usize>
where
    T: Eq + Hash + std::fmt::Debug,
{
    fn increment(&mut self, key: T, count: usize) {
        *self.entry(key).or_insert(0) += count;
    }

    fn decrement(&mut self, key: T, count: usize) {
        *self.entry(key).or_insert(0) -= count;
    }

    fn decrement_delete(&mut self, key: T, count: usize) {
        match *self.get(&key).unwrap() {
            x if x <= count => {
                self.remove(&key);
            }
            _ => *self.entry(key).or_insert(0) -= count,
        };
    }
}
