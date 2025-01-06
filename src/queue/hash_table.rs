/*!
 Hash Table: 基于数组的数据结构，它通过哈希函数将键（Key）映射到数组中的一个特定位置（桶或槽）来实现高效的数据存储和查找。
  1. 数组：存储数据的容器，也叫“桶”或“槽”。
  2. 哈希函数：将键（Key）映射到数组索引的函数。

  工作原理:
   1. 插入数据：当我们想要插入一个键值对（Key-Value）时，哈希函数首先会根据键计算出一个数组索引，确定存储位置。然后，将值存储在该位置
   2. 查找数据：当需要查找某个键对应的值时，哈希表同样使用哈希函数计算该键对应的数组索引，然后直接访问该位置，得到值
   3. 删除数据：删除数据时，哈希函数计算出该键的位置并将该位置的数据移除

  哈希冲突:
   由于哈希函数可能会将多个不同的键映射到同一个数组位置（索引），这就产生了“哈希冲突”
  解决方案:
   1. 链式哈希：每个数组位置存储一个链表（或其他数据结构），哈希冲突的元素会被存储在链表中。这样，多个键值对可以存储在同一个索引位置。
   2. 开放地址法：当发生哈希冲突时，哈希表会通过某种方式查找下一个空槽来存储冲突的数据。常见的探测方法有线性探测、二次探测等。

  优点:
    1. 查找效率高：在理想情况下，哈希表的查找时间复杂度为O(1)，即常数时间复杂度。
    2. 插入和删除操作高效：哈希表在插入和删除数据时也能提供较高的效率，时间复杂度为O(1)。

  缺点:
    1. 空间浪费：哈希表需要预先分配数组的空间，可能会导致内存浪费，特别是在哈希表的容量很大或者元素较少时。
    2. 哈希冲突处理复杂：尽管有解决冲突的方法，但在极端情况下，哈希冲突会导致性能下降，特别是在负载因子较高时。
    3. 哈希函数的设计困难：一个不合适的哈希函数会导致大量的冲突，从而影响哈希表的性能。

  应用场景:
   哈希表广泛应用于需要快速查找、插入、删除的数据结构中，如：
   1. 数据库索引：数据库中的索引通常使用哈希表来加速查找操作。
   2. 缓存：哈希表可以用来实现缓存机制（例如LRU缓存），快速访问已缓存的数据。
   3. 计数器和频率统计：通过哈希表，可以统计每个元素出现的次数。
   4. 集合操作：例如，判断一个元素是否在集合中存在等。

  线性探测的时间复杂度:
   1. 理想情况（低负载因子）：
      如果哈希表的 负载因子 （已存储元素数量 / 总容量）较低，哈希冲突少，线性探测的查找路径很短，插入、删除和查找的时间复杂度接近 O(1)。
   2. 最坏情况（高负载因子）
     当负载因子接近 1（即哈希表几乎满了）时，冲突显著增加，探测链变长。此时，线性探测的时间复杂度可能退化为 O(n)，因为在最坏情况下可能需要遍历整个表。

   如何减缓时间复杂度退化
   1. 控制负载因子：通过在负载因子达到一定阈值（如0.75）时触发扩容，降低哈希冲突的概率，从而缩短线性探测链的长度
   2. 选择更高效的哈希算法：确保哈希值分布均匀，减少冲突
   3. 使用其他冲突解决方法：如链地址法（用链表存储冲突元素）或开地址法的其他策略（如二次探测）

*/

use rand::{random, Rng};
use std::collections::LinkedList;
use std::fmt::{Debug, Display};
use std::hash::{DefaultHasher, Hash, Hasher};

// 最小容量, 最小值为 8 或 16, 8 是常见的默认值，因为它是 2 的幂（便于位运算优化）且能支持一定数量的元素, 16 则更适合需要较高初始性能的场景
const MIN_CAPACITY: usize = 8;

// 负载因子, 默认为 0.75
const LOAD_FACTOR: f32 = 0.75;

///  线性探查法(开放寻址法)
pub struct HashMap<K, V> {
    table: Vec<Option<(K, V)>>,
    valid_indices: Vec<usize>, // 记录非空槽位的索引
    capacity: usize,
    size: usize,
}

impl<K: Eq + Hash + Clone + Display, V: Clone + Debug> HashMap<K, V> {
    pub fn new(capacity: usize) -> HashMap<K, V> {
        Self {
            table: vec![None; capacity],
            capacity,
            valid_indices: Vec::new(),
            size: 0,
        }
    }

    /// 改变大小
    fn resize(&mut self, capacity: usize) {
        let mut table = vec![None; capacity];
        let mut entries = Vec::new();
        self.valid_indices.clear();

        for entry in self.table.iter_mut() {
            if let Some(kv) = entry.take() {
                entries.push(kv);
            }
        }

        // 将条目重新插入到新的表中
        for (key, value) in entries {
            // 重新计算哈希值并找到新的插入位置
            let mut index = self.hash(&key) % capacity;

            // 线性探测法解决哈希冲突
            while table[index].is_some() {
                index = (index + 1) % capacity; // 重新计算位置
            }
            table[index] = Some((key, value));
            self.valid_indices.push(index);
        }

        self.table = table;
        self.capacity = capacity;
    }

    /// 扩容
    pub fn expand(&mut self) {
        if (self.size / self.capacity) > LOAD_FACTOR as usize {
            self.resize(self.capacity * 2);
        }
    }

    /// 缩容
    pub fn shrink(&mut self) {
        if self.size > (self.capacity / 4) || self.capacity < MIN_CAPACITY {
            return;
        }

        self.resize(self.capacity / 2);
    }

    /// 增/改，复杂度 O(1)
    pub fn put(&mut self, key: K, value: V) {
        self.expand();
        let mut index = self.hash(&key);

        // 线性探测法解决哈希冲突
        while let Some((existing_key, _)) = &self.table[index] {
            if existing_key == &key {
                // 如果已经存在该键，直接更新值
                self.table[index] = Some((key, value));
                return;
            }

            index = (index + 1) % self.capacity;
        }

        self.table[index] = Some((key, value));
        self.size += 1;
        self.valid_indices.push(index);
    }

    /// 查，复杂度 O(1)
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let mut index = self.hash(&key);

        // 线性探测法查找
        let start = index; // 记录起始位置，防止无限循环
        loop {
            if let Some((existing_key, value)) = &self.table[index] {
                if existing_key == key {
                    return Some(value);
                }
            }

            index = (index + 1) % self.capacity;

            // 如果又回到起始位置，说明已经探测了所有位置
            if index == start {
                break;
            }
        }

        None
    }

    /// 删，复杂度 O(1)
    pub fn remove(&mut self, key: &K) -> bool {
        let mut index = self.hash(&key);

        // 线性探测法查找并删除
        let start = index; // 记录起始位置，防止无限循环
        loop {
            if let Some((existing_key, _)) = &self.table[index] {
                if existing_key == key {
                    self.table[index] = None;
                    self.size -= 1;
                    self.valid_indices.retain(|i| i != &index);
                    self.shrink();
                    return true;
                }
            }

            index = (index + 1) % self.capacity;
            // 如果又回到起始位置，说明已经探测了所有位置
            if index == start {
                break;
            }
        }

        false
    }

    /// 随机获取一个数
    pub fn get_random(&self) -> Option<(&K, &V)> {
        if self.valid_indices.is_empty() {
            return None;
        }

        let mut random = rand::thread_rng();
        let index = self.valid_indices[random.gen_range(0..self.valid_indices.len())];
        self.table[index].as_ref().map(|(k, v)| (k, v))
    }

    /// 哈希函数，把 key 转化成 table 中的合法索引
    /// 时间复杂度必须是 O(1)，才能保证上述方法的复杂度都是 O(1)
    pub fn hash(&mut self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    /// 打印
    pub fn print(&self) {
        if self.table.len() == 0 {
            return;
        }

        print!("[\n");
        for key in self.table.iter() {
            if let Some((key, value)) = key {
                print!("({}, {:?}), ", key, value);
            } else {
                print!("(None), ");
            }
        }
        print!("\n]");
        println!();
    }
}

/// 拉链法
pub struct HashTable<K, V> {
    table: Vec<LinkedList<(K, V)>>,
    capacity: usize,
    size: usize,
}

impl<K: Eq + Hash + Clone + Display, V: Clone + Debug> HashTable<K, V> {
    pub fn new(capacity: usize) -> HashTable<K, V> {
        Self {
            table: vec![LinkedList::new(); capacity],
            capacity,
            size: 0,
        }
    }

    /// 改变大小
    fn resize(&mut self, capacity: usize) {
        let mut table = vec![LinkedList::new(); capacity];
        let mut entries = Vec::new();

        let old_table = self.table.clone();
        for entry in old_table.iter() {
            if entry.is_empty() {
                continue;
            }
            entries.push(entry);
        }

        // 将条目重新插入到新的表中
        for bucket in entries {
            for (key, value) in bucket {
                // 重新计算哈希值并找到新的插入位置
                let mut index = self.hash(&key) % capacity;

                // 线性探测法解决哈希冲突
                while let Some((existing_key, _)) = table[index].front() {
                    if existing_key == key {
                        // 如果桶中已经有该键，则不需要重新计算位置
                        // 直接将新的值插入到该桶对应的 LinkedList 中
                        table[index].push_back((key.clone(), value.clone()));
                        break;
                    }

                    index = (index + 1) % capacity;
                }

                // 如果没有找到该键，直接插入
                if table[index].is_empty() || table[index].front().is_none() {
                    table[index].push_back((key.clone(), value.clone()));
                }
            }
        }

        self.table = table;
        self.capacity = capacity;
    }

    /// 扩容
    pub fn expand(&mut self) {
        if self.size > self.capacity / 2 {
            self.resize(self.capacity * 2);
        }
    }

    /// 缩容
    pub fn shrink(&mut self) {
        if self.size > (self.capacity / 4) || self.capacity < MIN_CAPACITY {
            return;
        }

        self.resize(self.capacity / 2);
    }

    /// 插入
    pub fn put(&mut self, key: K, value: V) {
        self.expand();
        let mut index = self.hash(&key);
        let start = index;

        loop {
            // 获取桶对应的链表
            let bucket = &mut self.table[index];
            for entry in bucket.iter_mut() {
                if entry.0 == key {
                    bucket.push_back((key.clone(), value.clone()));
                    return;
                }
            }

            // 如果未找到，则插入新的键值对
            if bucket.is_empty() {
                bucket.push_back((key.clone(), value.clone()));
                self.size += 1;
                return;
            } else {
                index = (index + 1) % self.capacity;
            }

            if start == index {
                break;
            }
        }
    }

    /// 查找
    pub fn get(&mut self, key: &K) -> Option<&LinkedList<(K, V)>> {
        let mut index = self.hash(&key);
        let start = index;
        loop {
            let bucket = &self.table[index];
            for entry in bucket.iter() {
                if entry.0 == key.clone() {
                    return Some(&bucket);
                }
            }

            index = (index + 1) % self.capacity;
            if index == start {
                break;
            }
        }

        None
    }

    /// 删除
    pub fn remove(&mut self, key: &K) -> bool {
        let mut index = self.hash(&key);
        let start = index;
        let mut i = -1;
        let mut x = 0;
        loop {
            let table = &mut self.table;
            for (_, entry) in table[index].iter().enumerate() {
                if entry.0 != key.clone() {
                    i = -1;
                } else {
                    i = index as i32;
                }

                break;
            }

            if i != -1 {
                if let Some(bucket) = table.get_mut(i as usize) {
                    *bucket = LinkedList::new();
                }

                self.size -= 1;
                self.shrink();
                return true;
            }

            if x == 0 {
                index = self.capacity - 1;
            }

            index = (index + 1) % self.capacity;
            x += 1;
            if index == start {
                break;
            }
        }

        false
    }

    /// 哈希函数，把 key 转化成 table 中的合法索引
    /// 时间复杂度必须是 O(1)，才能保证上述方法的复杂度都是 O(1)
    pub fn hash(&mut self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    /// 打印
    pub fn print(&self) {
        if self.table.len() == 0 {
            return;
        }

        print!("[\n");
        for bucket in self.table.iter() {
            print!("[");
            for (key, value) in bucket {
                print!("({}, {:?}), ", key, value);
            }
            print!("],");
        }
        print!("\n]");
        println!();
    }
}
