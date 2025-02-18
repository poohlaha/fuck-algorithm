/*!
  线性探查法, 开放寻址法的一种
*/

use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::hash::{DefaultHasher, Hash, Hasher};

// 负载因子, 默认为 0.75
const LOAD_FACTOR: f32 = 0.75;

// 最小容量, 最小值为 8 或 16, 8 是常见的默认值，因为它是 2 的幂（便于位运算优化）且能支持一定数量的元素, 16 则更适合需要较高初始性能的场景
const MIN_CAPACITY: usize = 8;

#[derive(Clone, Debug)]
enum Status {
    Empty,    // 空
    Occupied, // 已被占用
    Deleted,  // 删除
}
pub struct HashTable<K, V> {
    table: Vec<(Option<(K, V)>, Status)>, // 存储键值对, Status 存储每个槽位的状态（空、已占用、已删除）
    capacity: usize,                      // 哈希表的容量
    size: usize,                          // 哈希表中存储的元素个数
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Status::Empty, Status::Empty) => true,
            (Status::Occupied, Status::Occupied) => true,
            (Status::Deleted, Status::Deleted) => true,
            _ => false,
        }
    }
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq + Clone + Display + Debug,
    V: Clone + Display + Debug,
{
    pub fn new(capacity: usize) -> HashTable<K, V> {
        let capacity = capacity.next_power_of_two(); // 容量应该是2的幂次方
        let table = vec![(None, Status::Empty); capacity];
        Self {
            table,
            capacity,
            size: 0,
        }
    }

    pub fn resize(&mut self, capacity: usize) {
        let mut table = vec![(None, Status::Empty); capacity];

        self.capacity = capacity;
        for (pari, status) in self.table.iter() {
            if let Some((k, v)) = pari {
                let mut index = self.hash(&k);
                // 线性探测法解决哈希冲突
                while let Some(_) = &table[index].0 {
                    index = (index + 1) & (capacity - 1); // 重新计算位置
                }

                table[index] = (Some((k.clone(), v.clone())), status.clone());
            }
        }

        self.table = table;
    }

    /// 扩容
    fn expand(&mut self) {
        if (self.size as f32) / (self.capacity as f32) >= LOAD_FACTOR {
            self.resize(self.capacity * 2);
        }
    }

    /// 缩容
    fn shrink(&mut self) {
        if (self.size as f32) > ((self.capacity / 4) as f32) || self.capacity < MIN_CAPACITY {
            return;
        }

        self.resize(self.capacity / 2);
    }

    fn insert_internal(&mut self, key: K, value: V) {
        let mut index = self.hash(&key);

        // 使用线性探查寻找合适位置
        for _ in 0..self.capacity {
            let (pari, status) = &self.table[index];
            match status {
                Status::Empty | Status::Deleted => {
                    self.table[index] = (Some((key.clone(), value.clone())), Status::Occupied);
                    self.size += 1;
                    return;
                }
                Status::Occupied => {
                    // 如果键已存在，更新值
                    if let Some((k, _)) = pari {
                        if *k == key {
                            self.table[index] =
                                (Some((key.clone(), value.clone())), Status::Occupied);
                            return;
                        }
                    }
                }
            }

            index = (index + 1) & (self.capacity - 1); // 线性探查
        }
    }

    // 插入
    pub fn insert(&mut self, key: K, value: V) {
        self.expand();
        self.insert_internal(key, value);
    }

    // 查找
    pub fn search(&self, key: K) -> Option<V> {
        let mut index = self.hash(&key);

        for _ in 0..self.capacity {
            let (pari, status) = &self.table[index];

            if let Some((k, v)) = pari {
                match status {
                    Status::Occupied if *k == key => {
                        return Some(v.clone()); // 找到并返回值
                    }
                    Status::Empty => {
                        return None; // 找到空槽，表示该键不存在
                    }
                    _ => {} // 继续查找
                }
            }

            index = (index + 1) & (self.capacity - 1); // 线性探查
        }

        None
    }

    // 删除
    pub fn remove(&mut self, key: K) -> bool {
        let mut index = self.hash(&key);

        for _ in 0..self.capacity {
            let (pari, status) = &self.table[index];
            if let Some((k, _)) = pari {
                match status {
                    Status::Occupied if *k == key => {
                        self.table[index].0 = None;
                        self.table[index].1 = Status::Deleted;
                        self.size -= 1;

                        // 判断是否需要缩容
                        self.shrink();
                        return true;
                    }
                    Status::Empty => return false, // 找到空槽，说明没找到元素
                    _ => {}
                }
            }

            index = (index + 1) & (self.capacity - 1); // 线性探查
        }

        false
    }

    // 哈希函数
    pub fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::default();
        key.hash(&mut hasher);
        // hasher.finish() as usize % self.capacity
        // 因为 capacity 为 2 的幂次方, 可以用位运算符(&)代替取模(%), 范围为 0 ~ capacity - 1
        // 取 capacity - 1 只考虑哈希值的低位，并且正确地将哈希值限制在表的有效索引范围内
        hasher.finish() as usize & (self.capacity - 1)
    }

    /// 打印
    pub fn print(&self) {
        if self.table.len() == 0 {
            return;
        }

        print!("[");
        for (i, (pari, status)) in self.table.iter().enumerate() {
            print!("[");

            let status_str = match status {
                Status::Empty => "Empty",
                Status::Deleted => "Deleted",
                Status::Occupied => "Occupied",
            };

            if let Some((k, v)) = pari {
                print!("({}, {:?}, {})", k, v, status_str);
            }

            if i != self.table.len() - 1 {
                print!("],");
            }
        }
        print!("]");
        println!();
    }
}
