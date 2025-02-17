/*!
   队列, 通过数组实现, Rust 使用 VecDeque(双端队列) 实现, 另外也可以使用 `环形数组`
*/

use std::collections::VecDeque;

pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    // 向栈顶插入元素, 时间复杂度为 O(1)
    pub fn push(&mut self, data: T) {
        self.data.push_back(data);
    }

    // 从栈顶删除元素, 时间复杂度为 O(1)
    pub fn pop(&mut self) -> bool {
        self.data.pop_front().is_some()
    }

    // 查看栈顶元素, 时间复杂度为 O(1)
    pub fn peek(&mut self) -> Option<&T> {
        self.data.front()
    }

    // 返回栈中的元素个数, 时间复杂度为 O(1)
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
