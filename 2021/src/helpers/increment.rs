use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Increment<T, C> {
    fn increment(&mut self, key: T, count: C);
    fn decrement(&mut self, key: T, count: C);
    fn decrement_delete(&mut self, key: T, count: C);
}

impl<T, C> Increment<T, C> for HashMap<T, C>
where
    T: Eq + Hash + std::fmt::Debug + Clone,
    C: std::ops::AddAssign + std::ops::SubAssign + std::cmp::PartialOrd + Default + Copy,
{
    fn increment(&mut self, key: T, count: C) {
        *self.entry(key).or_default() += count;
    }

    fn decrement(&mut self, key: T, count: C) {
        *self.entry(key).or_default() -= count;
    }

    fn decrement_delete(&mut self, key: T, count: C) {
        *self.entry(key.clone()).or_default() -= count;
        // If the value is now back to zero, remove this entry
        if *self.get(&key).unwrap() == Default::default() {
            self.remove(&key);
        }
    }
}
