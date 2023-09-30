#[allow(warnings, dead_code)]
use std::collections::VecDeque;

// TODO use traits to cover any from u8 to u64 depending on limit
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

// TODO generics/trait bounds to cover any size from u8 u32 u64
// TODO tests
// TODO read C++ implementation
pub struct BitSet {
    data: u32,
}

impl BitSet {
    pub fn new() -> Self {
        Self { data: 0 }
    }
    pub fn set(&mut self, position: u32, set_bit_on: bool) {
        // zero-index
        if set_bit_on {
            self.data |= 1 << position;
        } else {
            self.data &= !(1 << position);
        };
    }
    pub fn get(&self, position: usize) -> bool {
        self.data & (1 << position) > 0
    }
    pub fn reset(&mut self) {
        self.data = 0;
    }
    pub fn is_empty(&self) -> bool {
        self.data == 0
    }

    pub fn size(&self) -> usize {
        std::mem::size_of_val(&self.data) * 8
    }
    // iterate starts from least significant bit
    pub fn iter(&self) -> BitSetIter {
        BitSetIter {
            bitset: self,
            position: 0,
        }
    }

    pub fn union(&self, other: &BitSet) -> BitSet {
        BitSet {
            data: self.data | other.data,
        }
    }

    pub fn intersection(&self, other: &BitSet) -> BitSet {
        BitSet {
            data: self.data & other.data,
        }
    }
    pub fn difference(&self, other: &BitSet) -> BitSet {
        BitSet {
            data: self.data & (!other.data),
        }
    }
}

pub struct BitSetIter<'a> {
    bitset: &'a BitSet,
    position: usize,
}

impl<'a> Iterator for BitSetIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.bitset.size() {
            return None;
        }
        let bit = self.bitset.get(self.position);
        self.position += 1;
        Some(bit)
    }
}
