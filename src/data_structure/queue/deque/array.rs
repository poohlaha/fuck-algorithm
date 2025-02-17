/*!
    双端队列, 通过 `环形数组` 实现
*/

use crate::data_structure::array::ring::Ring;
use std::fmt::Debug;

pub struct Array<T> {
    ring: Ring<T>,
}

impl<T: Clone + Debug> Array<T> {
    pub fn new() -> Self {
        Self { ring: Ring::new() }
    }

    // 入队操作: 添加元素到队列的尾部
    pub fn push_back(&mut self, element: T) {
        self.ring.add_last(element);
    }

    // 入队操作: 添加元素到队列的头部
    pub fn push_front(&mut self, element: T) {
        self.ring.add_first(element);
    }

    // 出队操作：从队列头部删除元素
    pub fn pop_front(&mut self) {
        self.ring.remove_first();
    }

    // 出队操作: 从队列尾部删除元素
    pub fn pop_back(&mut self) {
        self.ring.remove_last();
    }

    // 获取队列的头部元素
    pub fn front(&mut self) -> Option<&T> {
        self.ring.get_first()
    }

    // 获取队列的尾部元素
    pub fn back(&mut self) -> Option<&T> {
        self.ring.get_last()
    }

    // 检查队列是否为空
    pub fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }

    // 获取队列的大小
    pub fn size(&self) -> usize {
        self.ring.size()
    }

    pub fn print(&self) {
        self.ring.print();
    }
}
