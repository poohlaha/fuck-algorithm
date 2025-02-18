/*!
  开放寻址法, 当冲突发生时，尝试在哈希表中找到另一个空槽来存储冲突的元素
  状态:
  1. 标记槽位是否为空(Empty)
     - 未被使用, 将其标记为 `Empty`
     - 查找时碰到 `Empty`, 该位置没有被占用，并且所有后续位置也不能是有效的元素，因此我们可以停止查找过程
  2. 标记槽位是否已被占用(Occupied)
     - 存储了有效的键值对时, 将其标记为 `Occupied`
     - 在查找和插入操作时
       - 遇到一个 `Occupied` 状态的槽位，我们需要检查该槽位是否存储了我们想要的键。
         - 如果是，则返回相应的值
         - 如果不是，则继续探查下一个槽位
  3. 标记槽位是否已被删除(Deleted)
     - 删除操作后，标记为 `Deleted`
     - 后续的插入操作可以利用该槽位
     - 在查找时，我们需要跳过 `Deleted` 状态的槽位，因为它并不是空的，而是曾经被删除过的
   为什么需要 Deleted 状态？
   在开放寻址法中，删除操作不是简单地将槽位标记为空，而是需要标记为 Deleted
   - 避免误判
     - 如果一个槽位被删除后直接变为空(Empty)，在后续插入或查找时，当我们碰到空槽时可能会误认为该位置是一个有效的空槽。这会导致插入操作跳过已经被删除过的槽位，从而无法插入元素
   - 保持正确的探查序列
     - 在开放寻址法中，如果我们删除了一个元素并将其标记为空，我们在插入新元素时可能会误跳过已删除的槽位。通过将槽位标记为 Deleted，我们确保探查过程可以继续，并且能够填充已经被删除的位置
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
    table: Vec<Option<(K, V)>>, // 存储键值对
    status: Vec<Status>,        // 存储每个槽位的状态（空、已占用、已删除）
    capacity: usize,            // 哈希表的容量
    size: usize,                // 哈希表中存储的元素个数
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
        let table = vec![None; capacity];
        let status = vec![Status::Empty; capacity];
        Self {
            table,
            status,
            capacity,
            size: 0,
        }
    }

    pub fn resize(&mut self, capacity: usize) {
        let mut table = vec![None; capacity];
        let mut status = vec![Status::Empty; capacity];
        self.capacity = capacity;

        for (i, slot) in self.table.iter().enumerate() {
            if let Some((key, value)) = slot {
                let index = self.hash(key);
                table[index] = Some((key.clone(), value.clone()));
                status[index] = self.status[i].clone();
            }
        }

        self.table = table;
        self.status = status;
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

    // 插入
    pub fn insert(&mut self, key: K, value: V) {
        self.expand();

        let index = self.hash(&key);

        // 查找是否存在该键, 如果存在则更新
        while self.status[index] == Status::Occupied {
            if let Some((k, _)) = &self.table[index] {
                if k == &key {
                    self.table[index] = Some((key.clone(), value.clone()));
                    return;
                }
            }
        }

        self.table[index] = Some((key, value));
        self.status[index] = Status::Occupied;
        self.size += 1;
    }

    // 查找
    pub fn search(&self, key: K) -> Option<V> {
        let index = self.hash(&key);
        while self.status[index] != Status::Empty {
            if let Some((k, v)) = &self.table[index] {
                if k == &key {
                    return Some(v.clone());
                }
            }
        }

        None
    }

    // 删除
    pub fn remove(&mut self, key: K) -> bool {
        let index = self.hash(&key);
        while self.status[index] != Status::Empty {
            if let Some((k, _)) = &self.table[index] {
                if k == &key {
                    self.table[index] = None;
                    self.status[index] = Status::Deleted;
                    self.size -= 1;
                    self.shrink();
                    return true;
                }
            }
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
        for (i, bucket) in self.table.iter().enumerate() {
            print!("[");
            let status = self.status[i].clone();
            let status_str = match status {
                Status::Empty => "Empty",
                Status::Deleted => "Deleted",
                Status::Occupied => "Occupied",
            };

            if let Some((k, v)) = bucket {
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
