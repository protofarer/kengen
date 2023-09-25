use std::collections::VecDeque;

pub struct FixedSizeQueue {
    queue: VecDeque<u64>,
    limit: usize,
}

impl FixedSizeQueue {
    pub fn new(limit: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            limit,
        }
    }
    pub fn push(&mut self, value: u64) {
        if self.queue.len() >= self.limit {
            self.queue.pop_front();
        }
        self.queue.push_back(value);
    }
    pub fn avg(&self) -> Option<f64> {
        if self.queue.is_empty() {
            return None;
        }
        let sum: u64 = self.queue.iter().sum();
        Some(sum as f64 / self.queue.len() as f64)
    }
}
