/*!
  栈, 只能在某一端插入和删除元素。

  -->     -->
  |        |  栈顶
  |        |
  |        |
  |        |
  |        |
  |        |
  |_ _ _ _ |
*/
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// 向栈顶插入元素，时间复杂度 O(1)
    pub fn push(&mut self, element: T) {
        self.data.push(element);
    }

    /// 从栈顶删除元素，时间复杂度 O(1)
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// 查看栈顶元素，时间复杂度 O(1)
    pub fn peek(&mut self) -> Option<&T> {
        self.data.last()
    }

    /// 返回栈中的元素个数，时间复杂度 O(1)
    pub fn size(&mut self) -> usize {
        self.data.len()
    }
}
