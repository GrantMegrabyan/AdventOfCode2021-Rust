use std::collections::VecDeque;

pub struct SlidingWindow {
    pub window: VecDeque<i32>,
    pub sum: i32,
}

impl SlidingWindow {
    pub fn new(window_size: usize) -> Self {
        SlidingWindow {
            window: VecDeque::with_capacity(window_size),
            sum: 0,
        }
    }

    pub fn add(&mut self, item: i32) {
        if self.window.len() < self.window.capacity() {
            self.window.push_back(item);
            self.sum += item;
        } else {
            let last = self.window.pop_front().unwrap();
            self.window.push_back(item);
            self.sum = self.sum - last + item;
        }
    }

    pub fn is_full(&self) -> bool {
        self.window.capacity() == self.window.len()
    }
}
