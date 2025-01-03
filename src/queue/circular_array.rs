/*!
  双端队列: 允许在两端（队头和队尾）执行插入和删除操作。
  它是一种通用的数据结构，可以同时支持:
   1. 队列: (先进先出，FIFO) 的特性：在队尾插入，在队头删除。
   2. 栈: (后进先出，LIFO) 的特性：在同一端插入和删除。
  性能:
   1. 插入和删除操作的时间复杂度通常为 O(1)
   2. 访问特定位置的时间复杂度为 O(n)(与普通数组类似)
  实现方式:
   1. 基于动态数组(例如环形数组)
   2. 基于链表

  1. 基于环形数组
     1) 将一个固定大小的数组视为“环”，队列的逻辑两端（队头和队尾）通过模运算 mod 连接。
     2) 优势:
        a. 空间利用率高。
        b. 插入和删除操作高效
     3) 扩容: 需要复制现有数据到更大的数组。

  2. 基于双向链表
     1) 使用链表结构，每个节点包含指向前后节点的指针
     2) 队头和队尾的插入/删除操作直接修改指针
     3) 缺点: 需要额外的存储空间（指针占用）

   使用场景:
    1. 滑动窗口问题: 在给定数组中寻找特定窗口范围内的最大/最小值 | 使用双端队列可以高效地管理窗口中的元素
    2. 回文检查: 检查一个字符串是否是回文 | 使用双端队列可以从两端同时操作
    3. 调度器: 操作系统中的任务调度器需要动态管理任务队列 | 双端队列适合存储任务，并支持优先处理
    4. 缓冲区实现: 环形缓冲区（circular buffer）常用于实现输入输出缓冲区, 常用于存储流式数据, 它使用固定大小的缓冲区，在尾部插入数据并从头部取出数据。数据写入或读取到缓冲区末尾时，会循环到缓冲区的开头，从而实现“环形”的效果。
*/

use std::fmt::Debug;

pub struct CircularArray<T> {
    data: Vec<Option<T>>, // 环形数组
    front: usize,         // 队头指针
    rear: usize,          // 队尾指针
    capacity: usize,      // 容量
    size: usize,          // 当前大小
}

impl<T: Clone + Debug> CircularArray<T> {
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
    fn resize(&mut self, capacity: usize) {
        let mut data = vec![None; capacity]; // Vec::with_capacity(capacity): 这仅分配足够的内存，但向量的长度为 0，不包含任何元素
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

    /// 在头部添加元素, 时间复杂度 O(1)
    pub fn push_front(&mut self, data: T) {
        // 当满时, 扩容为原来的两倍
        if self.full() {
            self.resize(self.capacity * 2);
        }

        // front 环形前移
        self.front = (self.front + self.capacity - 1) % self.capacity;
        self.data[self.front] = Some(data);
        self.size += 1;
    }

    /// 在尾部添加元素, 时间复杂度 O(1)
    pub fn push_back(&mut self, data: T) {
        // 当满时, 扩容为原来的两倍
        if self.full() {
            self.resize(self.capacity * 2);
        }

        // 因为 rear 是开区间，所以是先赋值，再右移
        self.data[self.rear] = Some(data);
        self.rear = (self.rear + 1) % self.capacity; // 环形后移
        self.size += 1;
    }

    /// 删除头部元素, 时间复杂度 O(1)
    pub fn pop_front(&mut self) -> bool {
        if self.empty() {
            return false;
        }

        // 因为 front 是闭区间，所以先赋值，再右移
        self.data[self.front].take();
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        self.house();
        true
    }

    /// 删除尾部元素, 时间复杂度 O(1)
    pub fn pop_back(&mut self) -> bool {
        if self.empty() {
            return false;
        }

        // 因为 rear 是开区间，所以先左移，再赋值
        self.rear = (self.rear - 1) % self.capacity;
        self.data[self.rear].take();
        self.size -= 1;
        self.house();
        true
    }

    /// 获取头部元素, 时间复杂度 O(1)
    pub fn front(&self) -> Option<T> {
        if self.empty() {
            return None;
        }

        self.data[self.front].clone()
    }

    /// 获取尾部元素, 时间复杂度 O(1)
    pub fn rear(&self) -> Option<T> {
        if self.empty() {
            return None;
        }

        self.data[self.rear].clone()
    }

    /// 判断是否已满, 当 (rear + 1) % capacity == front 时，队列已满
    pub fn full(&self) -> bool {
        (self.rear + 1) % self.capacity == self.front
    }

    /// 当 front == rear 时，队列为空, front == rear == 0
    pub fn empty(&self) -> bool {
        self.size == 0
    }

    /// 获取大小
    pub fn size(&self) -> usize {
        self.size
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
