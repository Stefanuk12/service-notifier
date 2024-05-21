use std::ops::{Deref, DerefMut};

use arraydeque::ArrayDeque;

/// A circular buffer that removes the oldest item (index `0`) when full.
pub struct CircularBuffer<T, const CAP: usize>(pub ArrayDeque<T, CAP>);
impl<T, const CAP: usize> CircularBuffer<T, CAP> {
    /// Create a new [CircularBuffer].
    pub fn new() -> Self {
        Self(ArrayDeque::new())
    }

    /// Push an item to the buffer, removing the oldest item if the buffer is full.
    pub fn push(&mut self, item: T) {
        if self.0.is_full() {
            self.0.pop_front();
        }
        self.0.push_back(item).unwrap();
    }
}
impl<T, const CAP: usize> Deref for CircularBuffer<T, CAP> {
    type Target = ArrayDeque<T, CAP>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const CAP: usize> DerefMut for CircularBuffer<T, CAP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
