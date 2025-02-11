/*!
  环形数组:
  利用求模(%, 余数)运算，将普通数组变成逻辑上的环形数组，可以让我们用 O(1) 的时间在数组头部增删元素
  - 维护两指针: `start(指向第一个有效元素的索引)` 和 `end(指向最后一个有效元素的下一个位置索引)`
  - 当超出边界时(< 0 | >= arr.len()), 接着回到 头部或尾部
    - 左移: (self.start - 1 + self.capacity) % self.capacity
    - 右移: (self.start + 1) % self.capacity
  - 在数组头部添加和删除时移动 `start`
  - 在数组尾部添加和删除时移动 `end`
  - 区间: 左闭右开, [start, end)
*/

use std::fmt::Debug;

// 数组初始容量为 1
const INTI_CAP: usize = 1;

pub struct Ring<T> {
    data: Vec<Option<T>>, // 数据
    start: usize,         // 第一个有效元素的索引
    end: usize,           // 最后一个有效元素的下一个位置索引
    capacity: usize,      // 容量
    size: usize,          // 大小
}

impl<T: Clone + Debug> Ring<T> {
    pub fn new() -> Self {
        Self {
            data: vec![None; INTI_CAP],
            start: 0,
            end: 0,
            capacity: INTI_CAP,
            size: 0,
        }
    }

    // 扩容/收容
    // 当数组元素大小达到容量上限时，扩容为原来的 `2` 倍
    // 当数组元素大小缩减到容量的 `1/4` 时，缩容为原来的 `1/2`
    pub fn resize(&mut self, capacity: usize) {
        let mut data = vec![None; capacity];
        for i in 0..self.size {
            // 重新计算索引位置
            let index = (self.start + i) % self.capacity;
            data[i] = self.data[index].take();
        }

        self.data = data;
        self.capacity = capacity;
        self.start = 0; // 新队头索引
        self.end = self.size; // 新队尾索引
    }

    // 从队头写入数据
    pub fn add_first(&mut self, value: T) {
        self.scaling();

        // start 是闭区间，所以先左移，再赋值
        self.start = (self.start + self.capacity - 1) % self.capacity;
        self.data[self.start] = Some(value);
        self.size += 1;
    }

    // 从队尾写入数据
    pub fn add_last(&mut self, value: T) {
        self.scaling();

        // end 是开区间，所以直接再赋值后, 再左移
        self.data[self.end] = Some(value);
        self.end = (self.end + 1) % self.capacity;
        self.size += 1;
    }

    // 从队尾头删除
    pub fn remove_first(&mut self) {
        // 闭区间, 先删除后右移
        self.data[self.start] = None;
        self.start = (self.start + 1) % self.capacity;
        self.size -= 1;

        self.shrink();
    }

    // 从队尾删除
    pub fn remove_last(&mut self) {
        if self.size == 0 {
            return;
        }

        // 开区间, 先右移, 再删除
        self.end = (self.end + 1) % self.capacity;
        self.data[self.end] = None;
        self.size -= 1;

        self.shrink();
    }

    // 从队头读取数据
    pub fn get_first(&mut self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }

        self.data[self.start].as_ref()
    }

    // 从队尾读取数据
    pub fn get_last(&mut self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }

        // 开区间, 先左移(值在前一个)
        let index = (self.end + self.capacity - 1) % self.capacity;
        self.data[index].as_ref()
    }

    // 是否为空
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // 大小
    pub fn size(&self) -> usize {
        self.size
    }

    // 扩容
    // 当数组元素大小达到容量上限时，扩容为原来的 `2` 倍
    pub fn scaling(&mut self) {
        if self.size == self.capacity {
            self.resize(self.capacity * 2);
        }
    }

    // 收容
    // 当数组元素大小缩减到容量的 `1/4` 时，缩容为原来的 `1/2`
    pub fn shrink(&mut self) {
        if self.size > 0 && self.size == self.capacity / 4 {
            self.resize(self.capacity / 2);
        }
    }

    // 打印
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
