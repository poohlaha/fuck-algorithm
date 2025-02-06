/*!
 数组: 静态数组、动态数组
 静态数组: 一块连续的内存空间，我们可以通过索引来访问这块内存空间中的元素，这才是数组的原始形态。
 动态数组:
  - 当数组元素大小达到底层静态数组的容量上限时，扩容为原来的 `2` 倍
  - 当数组元素大小缩减到底层静态数组的容量的 `1/4` 时，缩容为原来的 `1/2`
  - 数组中插入新元素，新元素可能的插入位置并不是元素的索引，而是索引之间的空隙, 所以允许 index == size
*/

pub(crate) mod test;

use std::fmt::Debug;

// 数组初始容量为 1
const INTI_CAP: usize = 1;

#[derive(Debug)]
pub struct Array<T> {
    data: Vec<Option<T>>, // 数据
    size: usize,          // 大小
    cap: usize,           // 容量
}

impl<T: Clone + Debug + ToString> Array<T> {
    pub fn new() -> Self {
        Self {
            data: vec![None; INTI_CAP],
            size: 0,
            cap: INTI_CAP,
        }
    }

    // 初始化
    pub fn init(&mut self, cap: usize) {
        self.data = vec![None; cap];
        self.cap = cap;
        self.size = 0;
    }

    // 扩容/缩容
    // 当数组元素大小达到底层静态数组的容量上限时，扩容为原来的 `2` 倍
    // 当数组元素大小缩减到底层静态数组的容量的 `1/4` 时，缩容为原来的 `1/2`
    pub fn resize(&mut self) {
        if self.size == 0 {
            return;
        }

        let mut new_cap = self.cap;
        let n = self.data.len();
        // 上限, 扩容 2 倍
        if self.size == n {
            new_cap = n * 2;
        }

        // 1/4 时, 缩容为 1/2
        if self.size == ((n / 4) as f32).floor() as usize {
            new_cap = n / 2;
        }

        if new_cap == self.cap {
            return;
        }

        let mut new_data: Vec<Option<T>> = vec![None; new_cap];
        for i in 0..self.data.len() {
            new_data[i] = self.data[i].take();
        }

        self.cap = new_cap;
        self.data = new_data;
    }

    // 增
    pub fn add(&mut self, t: T) {
        self.resize();
        self.data[self.size] = Some(t);
        self.size += 1; // 大小 + 1
    }

    // 通过索引添加
    pub fn add_by_index(&mut self, t: T, index: usize) -> bool {
        // 检查数组是否越界
        // 数组中插入新元素，新元素可能的插入位置并不是元素的索引，而是索引之间的空隙, 所以允许 index == size
        if !self.check_position_index(index) {
            return false;
        }

        self.resize();
        // 数组搬运, 把当前索引位置的数据向后移一位, 从后向前移
        for i in (index..self.size).rev() {
            self.data[i + 1] = self.data[i].clone()
        }

        self.data[index] = Some(t);
        self.size += 1;
        true
    }

    // 修
    pub fn update(&mut self, t: T, index: usize) -> bool {
        if !self.check_element_index(index) {
            return false;
        }

        self.data[index] = Some(t);
        true
    }

    // 删
    pub fn delete(&mut self, index: usize) -> bool {
        if !self.check_element_index(index) {
            return false;
        }

        // 数组搬运, 把当前索引位置的数据向前移一位
        for i in index..self.size {
            self.data[i] = self.data[i + 1].clone();
        }

        self.data[self.size - 1] = None;
        self.size -= 1;
        true
    }

    // 查
    pub fn get(&self, index: usize) -> Option<&T> {
        if !self.check_element_index(index) {
            return None;
        }

        self.data[index].as_ref()
    }

    // 大小
    pub fn size(&mut self) -> usize {
        self.size
    }

    // 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // 数据展示
    pub fn display(&mut self) -> Vec<String> {
        if self.size() == 0 {
            return Vec::new();
        }

        let mut results: Vec<String> = Vec::new();
        for num in self.data.iter() {
            if let Some(num) = num {
                results.push(num.to_string());
            } else {
                results.push(String::new());
            }
        }

        results
    }

    // 检查数组是否越界(插入允许 index == size)
    fn check_position_index(&self, index: usize) -> bool {
        index >= 0 && index <= self.size
    }

    // 检查数组是事越界(修改, 删除不允许 index == size)
    fn check_element_index(&self, index: usize) -> bool {
        index >= 0 && index < self.size
    }
}
