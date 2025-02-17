/*!
    双端队列, 允许在队列两端进入插入和删除操作
    既可以作为栈（LIFO，后进先出）使用，也可以作为队列（FIFO，先进先出）使用
    - 在队列的 `前端` 插入或删除元素
    - 在队列的 `后端` 插入或删除元素
    1. 时间复杂度: O(1), 扩容的时间复杂度为 O(1)
    2. 空间复杂度: O(N)
    3. 实现
      - 头部插入(push_front): 在头部插入新元素
      - 尾部插入(push_back): 在头部插入新元素
      - 头部删除(push_front): 在头部删除元素
      - 尾部删除(push_back): 在尾部删除元素
      - 获取头部元素(front) 和 尾部元素(back): 访问头部和尾部元素
      - 检查是否为空(is_empty): 检查队列是否为空
      - 队列的大小(size): 获取队列的大小
*/

mod array;
mod link;
pub mod test;
