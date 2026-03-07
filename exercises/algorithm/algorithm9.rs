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
    T: Default + Clone + Copy,
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
        if self.is_empty() {
            self.items.push(value);
            self.count += 1;
        } else {
            // 将 value 添加到 items 末尾
            self.items.push(value);
            self.count += 1;
            // 从末尾开始"上浮",维护堆属性
            let mut current_idx = self.count;
            while current_idx > 1 {
                let current = self.items[current_idx];
                let parent_idx = self.parent_idx(current_idx);
                let parent = self.items[parent_idx];
                // 比较当前节点与父节点               
                // 如果当前节点更"优先"，交换
                if (self.comparator)(&current, &parent) {
                    self.items[parent_idx] = current;
                    self.items[current_idx] = parent;
                    // 更新当前节点
                    current_idx = parent_idx;
                } else {
                    break;
                }
            }
        }
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
        // 先取左子节点
        let left = self.left_child_idx(idx);
        let left_val = &self.items[left];
        // 如果右子节点存在且更"优先"，则返回右子节点
        let right = self.right_child_idx(idx);
        if let Some(right_val) = self.items.get(right) {
            if (self.comparator)(right_val, left_val) {
                return right;
            }
        }
        // 否则返回左子节点
        left
    }
}

// impl<T> Heap<T>
// where
//     T: Default + Ord,
// {
//     /// Create a new MinHeap
//     pub fn new_min() -> Self {
//         Self::new(|a, b| a < b)
//     }

//     /// Create a new MaxHeap
//     pub fn new_max() -> Self {
//         Self::new(|a, b| a > b)
//     }
// }

impl<T> Iterator for Heap<T>
where
    T: Default + Clone + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            // 保存堆顶元素 (items[1])
            let top = self.items[1];
            // 将末尾元素移到堆顶
            self.items[1] = self.items[self.len()];
            self.count -= 1;
            self.items.pop();// pop掉多余元素
            // 从根节点开始"下沉", 维护堆属性
            let mut current_idx = 1;
            while current_idx <= self.len() && self.children_present(current_idx) {
                // 找到更"优先"的子节点
                let child_idx = self.smallest_child_idx(current_idx);
                let child = self.items[child_idx];
                let current = self.items[current_idx];
                // 如果子节点比当前节点更"优先"，交换
                if (self.comparator)(&child, &current) {
                    self.items[child_idx] = current;
                    self.items[current_idx] = child;
                    // 更新当前节点
                    current_idx = child_idx;
                } else {
                    break;
                }
            }
            Some(top)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone + Copy,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone + Copy,
    {
        Heap::new(|a, b| a > b)
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
        assert_eq!(heap.next(), Some(1));
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
