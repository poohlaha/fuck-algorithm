/// 跳表, 支持多线程并发访问
use rand::Rng;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

// 跳表最大层数
const MAX_LEVEL: usize = 4;

// 每一层最大节点数
const PER_MAX_LEVEL: usize = 5;

#[derive(Debug)]
pub struct Node<T> {
    value: Option<T>,
    forwards: Vec<Option<Arc<Mutex<Node<T>>>>>, // 存储 `指向不同层的下一个节点` 的指针(多层指针)
}

impl<T> Node<T> {
    fn new(value: Option<T>, level: usize) -> Self {
        Self {
            value,
            forwards: vec![None; level],
        }
    }

    fn create(value: Option<T>, level: usize) -> Arc<Mutex<Node<T>>> {
        Arc::new(Mutex::new(Self::new(value, level)))
    }
}

// RwLock 允许多个线程同时读取，但写入时仍然需要独占锁
pub struct Skip<T> {
    head: Arc<Mutex<Node<T>>>, // 虚拟头节点, 不存储数据
    level: RwLock<usize>,      // 当前跳表最大层数
    max_level: usize,          // 最大层数
    size: RwLock<usize>,       // 当前节点数量
}

impl<T: Debug + Clone + Ord> Skip<T> {
    pub fn new(level: Option<usize>) -> Self {
        let max_level = level.unwrap_or(MAX_LEVEL);
        Self {
            head: Node::create(None, max_level),
            level: RwLock::new(1),
            max_level,
            size: RwLock::new(0),
        }
    }

    // 插入
    pub fn add(&mut self, value: T) {
        // 1. 定义 `update` 数组, 用于存储 `每一层的前驱节点`
        let mut update = vec![None; self.max_level];
        let mut current = Arc::clone(&self.head);

        // 从 1.1 `head` 开始，找到每一层的前驱节点, 从高层向低层查找
        {
            let level = *self.level.read().unwrap();
            for i in (0..level).rev() {
                let mut next_current = Arc::clone(&current); // 用于存储下一个 current，初始值为当前 current
                loop {
                    {
                        let mut current_ref = current.lock().unwrap();
                        let mut updated = false;
                        if let Some(forward) = current_ref.forwards.get(i).and_then(|f| f.as_ref())
                        {
                            let forward_ref = forward.lock().unwrap();
                            if forward_ref.value < Some(value.clone()) {
                                next_current = forward.clone();
                                updated = true;
                            }
                        }

                        if !updated {
                            break;
                        }
                    }

                    current = Arc::clone(&next_current);
                }

                current = next_current;
                // 1. 2 存储每一层的前驱节点到 `update` 数组
                // `update[i]` 代表第 `i + 1` 层的前驱节点，用于连接新插入的节点
                update[i] = Some(Arc::clone(&current));
            }
        }

        // 2. 随机决定新节点的层数
        let new_level = self.random_level();

        // 3.1 如果新节点的层数超过最高 > 当前层数(new_level > self.level)
        {
            let mut level = self.level.write().unwrap();
            if new_level > *level {
                // 3.2 把超出部分指向 `新的虚拟头节点`, 即为 None, 因为它们没有任何元素
                for i in *level..new_level {
                    update[i] = Some(Arc::clone(&self.head));
                }

                // 3.3 设置当前层数等于新节点的层数, self.level = new_level
                *level = new_level;
            }
        }

        // 4. 创建新节点
        let new_node = Node::create(Some(value), self.max_level);

        // 5. 从高层开始插入, 设置 `forwards` 指针
        for i in 0..new_level {
            if let Some(prev) = &update[i] {
                let mut prev_lock = prev.lock().unwrap();
                // 5.1 设置当前层新节点的下一个节点指针 = 当前层的前驱节点下一个节点指针
                new_node.lock().unwrap().forwards[i] = prev_lock.forwards[i].take();

                // 5.2 设置前驱节点的下一个节点指针 = 新节点
                prev_lock.forwards[i] = Some(Arc::clone(&new_node));
            }
        }

        *self.size.write().unwrap() += 1;
    }

    // 查找
    pub fn get(&self, value: T) -> bool {
        let mut current = Arc::clone(&self.head);
        let mut cur = Arc::clone(&current);

        // 从高层开始查找
        {
            let level = *self.level.read().unwrap();
            for i in (0..level).rev() {
                let mut next_current = Arc::clone(&current); // 用于存储下一个 current，初始值为当前 current
                loop {
                    {
                        let mut current_ref = current.lock().unwrap();
                        let mut updated = false;
                        if let Some(forward) = current_ref.forwards.get(i).and_then(|f| f.as_ref())
                        {
                            let forward_ref = forward.lock().unwrap();
                            if forward_ref.value < Some(value.clone()) {
                                next_current = forward.clone();
                                updated = true;
                            }
                        }

                        if !updated {
                            break;
                        }
                    }

                    current = Arc::clone(&next_current);
                }

                current = next_current;
            }
        }

        if let Some(next) = &current.lock().unwrap().forwards[0] {
            return next.lock().unwrap().value == Some(value);
        }

        false
    }

    // 删除
    pub fn remove(&mut self, value: T) -> bool {
        // 定义 `update` 数组, 用于存储 `每一层的前驱节点`
        let mut update = vec![None; self.max_level];

        let mut current = self.head.clone();
        let mut found = false;

        // 从 1.1 `head` 开始，找到每一层的前驱节点, 从高层向低层查找
        {
            let level = *self.level.read().unwrap();
            for i in (0..level).rev() {
                let mut next_current = Arc::clone(&current); // 用于存储下一个 current，初始值为当前 current
                loop {
                    {
                        let mut current_ref = current.lock().unwrap();
                        let mut updated = false;
                        if let Some(forward) = current_ref.forwards.get(i).and_then(|f| f.as_ref())
                        {
                            let forward_ref = forward.lock().unwrap();
                            if forward_ref.value < Some(value.clone()) {
                                next_current = forward.clone();
                                updated = true;
                            }
                        }

                        if !updated {
                            break;
                        }
                    }

                    current = Arc::clone(&next_current);
                }

                current = next_current;

                // 1. 2 存储每一层的前驱节点到 `update` 数组
                // `update[i]` 代表第 `i + 1` 层的前驱节点，用于连接新插入的节点
                update[i] = Some(Arc::clone(&current));
            }
        }

        let next = current.lock().unwrap().forwards[0].clone();
        if let Some(next) = next {
            if next.lock().unwrap().value != Some(value.clone()) {
                return false;
            }

            found = true;

            let mut level = *self.level.read().unwrap();
            // 从高层开始更新 forward 指针, 删除节点
            for i in (0..level).rev() {
                if let Some(prev) = &update[i] {
                    let mut prev_lock = prev.lock().unwrap();
                    let next_forward = prev_lock.forwards[i].clone();
                    if let Some(forward) = next_forward.as_ref() {
                        if forward.lock().unwrap().value == Some(value.clone()) {
                            prev_lock.forwards[i] = next.lock().unwrap().forwards[i].take();
                        }
                    }
                }

                // 更新跳表层数, 确保跳表不会有冗余的空层
                // 检查跳表的最高层（level）是否有多余的空层，如果有，就会逐渐减少跳表的层数，直到没有空的层为止。
                while level > 1 && self.head.lock().unwrap().forwards[i].is_none() {
                    let mut level_lock = self.level.write().unwrap();
                    *level_lock -= 1;
                    level = *level_lock; // 更新 level 值
                }
            }
        }

        if found {
            *self.size.write().unwrap() -= 1;
        }

        found
    }

    // 先删除旧值，再插入新值
    pub fn update(&mut self, value: T, new_value: T) -> bool {
        if self.remove(value) {
            self.print();
            self.add(new_value);
            return true;
        }
        false
    }

    // 生成随机层数
    fn random_level(&self) -> usize {
        let mut rng = rand::thread_rng();
        let mut level = 1;
        // 生成一个 50% 概率的布尔值(每一层向上减半, 用于决定每个节点是否会“升到”下一层)
        // 表的层数通常是通过抛硬币（或者类似的随机过程）来决定的
        // 使用 rng.gen_bool(0.5) 会导致要么在 `第一层`, 要么在 `最高层`, 导致缺失中间层
        /*
        let value = rng.gen_bool(0.5);
        while value && level < self.max_level {
            level += 1;
        }
         */

        /*
        while rng.gen_range(0.0..1.0) < 0.5 && level < self.max_level {
            level += 1;
        }
         */

        // 概率逐渐递减
        let mut probability = 0.5;
        while rng.gen_bool(probability) && level < self.max_level {
            level += 1;
            probability *= 0.75; // 递减概率
        }

        // println!("level: {}", level);
        level
    }

    // 打印
    pub fn print(&self) {
        let level = *self.level.read().unwrap();

        for i in (0..level).rev() {
            let mut current = Arc::clone(&self.head);

            print!("Level {}: head", i + 1);
            let mut next_current = Arc::clone(&current); // 用于存储下一个 current，初始值为当前 current
            loop {
                {
                    let mut current_ref = current.lock().unwrap();
                    let mut updated = false;
                    if let Some(next) = current_ref.forwards.get(i).and_then(|f| f.as_ref()) {
                        let forward_ref = next.lock().unwrap();
                        print!(" -> {:?}", forward_ref.value);
                        next_current = next.clone();
                        updated = true;
                    }

                    if !updated {
                        break;
                    }
                }

                current = Arc::clone(&next_current);
            }

            current = next_current;

            print!(" -> None");
            println!()
        }
    }

    #[allow(dead_code)]
    pub fn print_update(&self, update: &Vec<Option<Rc<RefCell<Node<T>>>>>) {
        println!("update:");
        for i in 0..update.len() {
            let mut current = Arc::clone(&self.head);

            print!("Update {}: ", i + 1);
            let mut next_current = Arc::clone(&current); // 用于存储下一个 current，初始值为当前 current
            loop {
                {
                    let mut current_ref = current.lock().unwrap();
                    let mut updated = false;
                    if let Some(next) = current_ref.forwards.get(i).and_then(|f| f.as_ref()) {
                        let forward_ref = next.lock().unwrap();
                        print!(" -> {:?}", forward_ref.value);
                        next_current = next.clone();
                        updated = true;
                    }

                    if !updated {
                        break;
                    }
                }

                current = Arc::clone(&next_current);
            }
            print!(" -> None");
            println!()
        }
    }
}
