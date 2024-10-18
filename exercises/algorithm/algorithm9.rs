/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        self.bubble_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if!self.children_present(idx) {
            return 0;
        }
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        if right_idx > self.count {
            return left_idx;
        }
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            return left_idx;
        } else {
            return right_idx;
        }
    }

    fn bubble_up(&mut self, idx: usize) {
        if idx == 1 {
            return;
        }
        let parent_idx = self.parent_idx(idx);
        if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
            self.items.swap(idx, parent_idx);
            self.bubble_up(parent_idx);
        }
    }
    fn sift_down(&mut self, idx: usize) {
        let mut smallest = idx;
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        if left_idx <= self.count && (self.comparator)(&self.items[left_idx], &self.items[smallest]) {
            smallest = left_idx;
        }
        if right_idx <= self.count && (self.comparator)(&self.items[right_idx], &self.items[smallest]) {
            smallest = right_idx;
        }
        if smallest!= idx {
            self.items.swap(idx, smallest);
            self.sift_down(smallest);
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let min = self.items[1].clone();
        self.items[1] = self.items[self.count].clone();
        self.count -= 1;
        self.sift_down(1);
        Some(min)
    }

    
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new_min()
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new_max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(11));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}