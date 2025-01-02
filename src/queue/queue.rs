/*!
    队列, 先进先出, 队列只允许在队尾插入元素，在队头删除元素

    队头                   队尾
       --------------------
     <--                   <--
       --------------------
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

    /// 向队尾插入元素，时间复杂度 O(1)
    pub fn push(&mut self, element: T) {
        self.data.push_back(element);
    }

    /// 从队头删除元素，时间复杂度 O(1)
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    ///  查看队头元素，时间复杂度 O(1)
    pub fn peek(&self) -> Option<&T> {
        self.data.front()
    }

    /// 返回队列中的元素个数，时间复杂度 O(1)
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
