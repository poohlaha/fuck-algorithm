/*!
  队列, 通过链表实现
*/

pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // 向栈顶插入元素, 时间复杂度为 O(1)
    pub fn push(&mut self, data: T) {
        self.data.push(data);
    }

    // 从栈顶删除元素, 时间复杂度为 O(1)
    pub fn pop(&mut self) -> bool {
        self.data.pop().is_some()
    }

    // 查看栈顶元素, 时间复杂度为 O(1)
    pub fn peek(&mut self) -> Option<&T> {
        self.data.last()
    }

    // 返回栈中的元素个数, 时间复杂度为 O(1)
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
