/*!
环形数组(循环数组或循环缓冲区), 它在逻辑上被视为一个首尾相连的数组，首元素和尾元素在逻辑上形成一个环。环形数组常用于实现固定大小的队列或缓冲区。
1. 固定大小：环形数组的大小是固定的，使用时不能动态扩展
2. 首尾相连：当尾部指针达到数组的最后一个位置后，如果再插入元素，指针会绕回到数组的开头
3. 两个指针:
   头指针(front): 指向队列的第一个有效元素
   尾指针(rear): 指向队列最后一个有效元素的下一个位置（用于插入新元素）
4. 判空与判满:
   判空: 当 front == rear 时，队列为空
   判满: 当 (rear + 1) % capacity == front 时，队列已满。

ps: 利用求模（余数）运算，将普通数组变成逻辑上的环形数组, 利用求模（余数）运算，将普通数组变成逻辑上的环形数组，可以让我们用 O(1) 的时间在数组头部增删元素。

举例:
 假设我们有一个容量为 5 的环形数组，用来模拟队列。
 初始化:
 数组: [_, _, _, _, _]
  front: 0
  rear: 0

 1. 插入1:
 数组: [1, _, _, _, _]
  front: 0
  rear: 1

 2. 插入 2 和 3:
 数组: [1, 2, 3, _, _]
  front: 0
  rear: 3

 3. 删除队头:
 数组: [_, 2, 3, _, _]
  front: 1
  rear: 3

 4. 插入 4 和 5:
 数组: [_, 2, 3, 4, 5]
  front: 1
  rear: 0
 ps: 此时 rear 指针“绕”回到了数组的起始位置（形成环状）

 5. 插入 6:
   队列已满，不能再插入新元素。
  (front == (rear + 1) % capacity)
  => (1 == (0 + 1) % 5)
  => (1 == 1) => 队列已满

 6. 删除队头:
 数组: [_, _, 3, 4, 5]
  front: 2
  rear: 0

 在第 5 步时，虽然数组物理上还有空位，但根据 `满队列的逻辑规则`，尾指针不能与头指针重合，因此队列已满。这是环形数组的一种常见设计，目的是确保状态判断的简单和高效

 (rear + 1) % capacity 和 (rear + 1) & (capacity - 1):
 (rear + 1) % capacity:
    1. 任何容量值的环形数组（任意整数）
    2. 通过取模运算，将索引限制在 [0, capacity - 1] 范围内
    3. 取模运算的性能相对较慢，尤其在高性能场景中（因为除法运算比位运算更耗时）

 (rear + 1) & (capacity - 1):
   1. 仅适用于 容量为 2 的幂次方（power of 2） 的数组（如 2, 4, 8, 16, ...）
   2. 利用二进制按位与运算，自动将索引限制在 [0, capacity - 1] 范围内。
   3. 性能非常高，按位与运算比取模运算快。

 注: `双端队列` 才可以队头和队尾添加
 双端队列应用场景:
 1. 模拟栈和队列的混合操作。
 2. 实现滑动窗口、双端搜索等。

 ps:
  普通队列（Queue）以 队尾插入、队头删除 为核心操作，是为了实现 FIFO 的特性。
  如果需要在队头添加元素，可以使用 双端队列（Deque），它既支持队头操作，也支持队尾操作。

 环形数组的区间通常被定义为 左闭右开 `[front,rear)`，是为了简化索引的计算和管理
*/

use std::fmt::Debug;

/// 全封闭
pub struct CircularBuffer<T> {
    data: Vec<Option<T>>, // 存储数据的底层容器
    front: usize,         // 头指针（读取位置）
    rear: usize,          // 尾指针（写入位置）
    capacity: usize,      // 容量
    size: usize,          // 当前大小
}

impl<T: Clone + Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![None; capacity],
            front: 0,
            rear: 0,
            capacity,
            size: 0,
        }
    }

    /// 自动扩容
    pub fn resize(&mut self, capacity: usize) {
        let mut data = vec![None; capacity]; // V
        for i in 0..self.size {
            let index = (self.front + i) % self.capacity;
            data[i] = self.data[index].take();
        }

        self.data = data;
        self.front = 0; // 新队头索引
        self.rear = self.size; // 新队尾索引
        self.capacity = capacity; // 更新容量
    }

    /// 自动收容, 如果元素数量减少到原大小的四分之一，则减小大小为一半
    pub fn house(&mut self) {
        if self.size > 0 && self.size == self.capacity / 4 {
            self.resize(self.capacity / 2);
        }
    }

    /// 从队尾写入数据
    pub fn push(&mut self, data: T) {
        if self.full() {
            self.resize(self.capacity * 2);
        }

        self.data[self.rear] = Some(data);
        self.rear = (self.rear + 1) % self.capacity;
        self.size += 1;
    }

    /// 从队头读取数据
    pub fn pop(&mut self) -> Option<T> {
        let value = self.data[self.front].take();
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        self.house();
        value
    }

    /// 判断是否已满, 当 s self.size == self.capacity 时，队列已满
    pub fn full(&mut self) -> bool {
        self.size == self.capacity
    }

    /// 当 self.size == 0 时，队列为空
    pub fn empty(&mut self) -> bool {
        self.size == 0
    }

    /// 打印
    pub fn print(&self) {
        let data = self.data.clone();
        print!("[");
        for value in data.iter() {
            print!("{:?},", value);
        }
        print!("]");
        println!();
    }
}
