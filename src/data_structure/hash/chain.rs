/*!
  通过 `链式地址法(拉链法)` 实现, 通过 `table` 中存储 `链表`
*/
use std::collections::LinkedList;
use std::fmt::{Debug, Display};
use std::hash::{DefaultHasher, Hash, Hasher};

// 负载因子, 默认为 0.75
const LOAD_FACTOR: f32 = 0.75;

// 最小容量, 最小值为 8 或 16, 8 是常见的默认值，因为它是 2 的幂（便于位运算优化）且能支持一定数量的元素, 16 则更适合需要较高初始性能的场景
const MIN_CAPACITY: usize = 8;

pub struct HashTable<K, V> {
    table: Vec<LinkedList<(K, V)>>, // 每个槽位用链表存储多个键值对
    capacity: usize,                // 哈希表的容量
    size: usize,                    // 哈希表中存储的元素个数
}

impl<K, V> HashTable<K, V>
where
    K: Eq + Hash + Debug + Clone + Display,
    V: Debug + Clone + Display,
{
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two(); // 容量应该是2的幂次方
        let table = vec![LinkedList::new(); capacity];
        Self {
            table,
            capacity,
            size: 0,
        }
    }

    fn resize(&mut self, capacity: usize) {
        let mut table = vec![LinkedList::new(); capacity];

        self.capacity = capacity;
        for list in self.table.iter() {
            for (key, value) in list.iter() {
                let index = self.hash(key);
                table[index].push_back((key.clone(), value.clone()));
            }
        }

        self.table = table;
    }

    /// 扩容
    fn expand(&mut self) {
        if ((self.size as f32) / (self.capacity as f32)) >= LOAD_FACTOR {
            self.resize(self.capacity * 2);
        }
    }

    /// 缩容
    fn shrink(&mut self) {
        if self.size > (self.capacity / 4) || self.capacity < MIN_CAPACITY {
            return;
        }

        self.resize(self.capacity / 2);
    }

    // 插入
    pub fn insert(&mut self, key: K, value: V) {
        self.expand();

        let index = self.hash(&key);
        let list = &mut self.table[index];

        // 查找是否存在该键, 如果存在则更新
        for pair in list.iter_mut() {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }

        // 不存在, 则插入
        list.push_back((key, value));
        self.size += 1;
    }

    // 删除
    pub fn remove(&mut self, key: K) -> bool {
        let index = self.hash(&key);
        let list = &mut self.table[index];

        let mut new_list: LinkedList<(K, V)> = LinkedList::new();
        let mut found = false;
        for (k, v) in list.iter() {
            if key != *k {
                new_list.push_back((k.clone(), v.clone()));
            } else {
                found = true
            }
        }

        if found {
            list.clear();
            *list = new_list;
            self.size -= 1;
            self.shrink();
            return true;
        }

        false
    }

    // 查找
    pub fn search(&self, key: K) -> Option<&V> {
        let index = self.hash(&key);
        let list = &self.table[index];
        for pair in list.iter() {
            if pair.0 == key {
                return Some(&pair.1);
            }
        }

        None
    }

    // keys
    pub fn keys(&self) -> Vec<K> {
        let mut keys: Vec<K> = vec![];
        for i in 0..self.capacity {
            let data = self.table[i].clone();
            for pair in data.iter() {
                keys.push(pair.0.clone());
            }
        }
        keys
    }

    // 哈希函数
    pub fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        // hasher.finish() as usize % self.capacity
        // 因为 capacity 为 2 的幂次方, 可以用位运算符(&)代替取模(%), 范围为 0 ~ capacity - 1
        // 取 capacity - 1 只考虑哈希值的低位，并且正确地将哈希值限制在表的有效索引范围内
        hasher.finish() as usize & (self.capacity - 1)
    }

    pub fn print(&self) {
        print!("[\n");

        for bucket in self.table.iter() {
            print!("[");
            for (key, value) in bucket {
                print!("({:?}, {:?}), ", key, value);
            }
            print!("],");
        }

        print!("\n]");
        println!();
    }
}
