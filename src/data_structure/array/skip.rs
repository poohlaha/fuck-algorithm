/*!
 跳表(Skip List)
 一种用于快速查找的随机化数据结构，常用于替代平衡树（如 AVL 树或红黑树）。
 它是一种多层次的链表结构，其中每个节点不仅在当前层次有指向下一个节点的指针，还有指向上层的指针，使得在查找时可以跳跃过一些不必要的节点，从而加速查找过程。
 1. 基本原理
    - 层次结构
       跳表包含`多个层次`，每一层都是一个`有序链表`。最低层（第0层）包含所有元素，其他层次逐渐减少。每一层的链表都是从上一层的节点中 `随机选择一部分节点` 构成的。
    - 节点结构
      每个节点包含多个指针
      - forward[i]：指向`当前层次的下一个节点`。`i` 表示当前节点在第 `i` 层的指针
      - 每个节点在 `较高层`次有 `更少` 的指针。每个节点只有在 `满足一定的概率` 条件下才会出现在更高的层次上
    - 插入
      - 当插入新节点时，会决定该节点在哪些层次中存在
      - 通常的做法是通过 `随机选择` 来决定节点出现的层数
      - 新节点会首先在`底层插入`，然后 `向上逐层` 决定是否将该节点 `提升` 到更高层次
    - 查找
      - 查找操作从跳表的 `最高层` 开始，尝试通过比较节点的值来跳过一些节点，直到找到目标节点或者降到最低层继续查找
      - 由于 `每一层的节点` 都是 `有序` 的，因此可以通过 `逐层向右跳跃` 来加速查找
    - 删除
      - 删除节点时，需要在 `每一层上找到` 并移除该节点

  2. 时间复杂度
    - 查找
      查找操作具有对数时间复杂度 `O(logN)`，比线性链表 `O(N)` 快得多
    - 插入
      插入操作的复杂度也是 `O(logN)`，插入过程中需要调整指针并可能提升节点
    - 删除
      删除操作的复杂度也是 `O(logN)`

   3. 优势
      - 简单易实现: 跳表的实现比红黑树等平衡树简单得多，尤其是在涉及到并发操作时
      - 支持快速查找: 由于跳表的多层结构，查找速度接近 `O(logN)`, 非常高效
      - 动态调整: 跳表能够动态适应数据的变化，无需复杂的平衡调整

   4. 跳表层数生成
      - 每个元素的层数是 `随机分配`的，常用的策略是
        - 几率递减的随机层数：每一层上的节点大约是下一层的 `1/2`，即 每个节点在当前层继续向上扩展的概率为 `50%`
        - 层数计算：新插入的元素的层数通常是 `随机生成` 的，直到达到 `MAX_LEVEL` 或者 `不满足随机条件`
          - 层数 = 1 的概率是 `50%`
          - 层数 = 2 的概率是 `25%`
          - 层数 = 3 的概率是 `12.5%`
          - 层数 = k 的概率是 `(1 / 2)^(k - 1)`

    5. 跳表最大层数的计算
       - 理论计算
         - `N` 是跳表的元素总数, `P` 是升层概率(常见值是 `0.5`), `MAX_LEVEL = log_{1/p}(N)`
           - 如果 `p = 0.5`, 那么: MAX_LEVEL = log2(N)
           - 数据量: 100, MAX_LEVEL = 7
           - 数据量: 1000, MAX_LEVEL = 10
           - 数据量: 10000, MAX_LEVEL = 14
           - 数据量: 1000000, MAX_LEVEL = 20
         - 当 `N` 在 `百万级` 时, `MAX_LEVEL = 20` 是合适的
         - 确保查询时间复杂度稳定在 `O(logN)`

       - 经验法则
         - 对于一般应用, 经验公式:
           - MAX_LEVEL = ceil(log2(N))
         - 小规模数据（1K-10K）：取 MAX_LEVEL = 10 左右
         - 中等规模（百万级）：取 MAX_LEVEL = 16 ~ 20
         - 超大规模（上亿级别）：取 MAX_LEVEL = 32
         - 通常，固定一个 `合理的最大层数`（如 MAX_LEVEL = 16 或 32），可以适用于 `大多数情况`。

       - 其他优化策略
         - 动态调整 `MAX_LEVEL`
           - 可以在插入元素时，动态调整 `MAX_LEVEL`，而不是固定 `MAX_LEVEL`
           - 例如：每次 MAX_LEVEL = log_2(N)，可以通过统计当前跳表的元素数量 N 来调整 MAX_LEVEL
         - 不同概率 `p` 影响
           - 常见的概率 `p = 0.5`，也可以调整为 `p = 1/e ≈ 0.37`，能减少层数，同时保持良好搜索效率
    参考: https://writings.sh/post/data-structure-skiplist

    插入步骤:
     - 定义 `update` 数组, 用于存储 `每一层的前驱节点`
       `update[i]` 代表第 `i + 1` 层的前驱节点，用于连接新插入的节点
       - 从 `head` 开始，找到每一层的前驱节点, 从高层向低层查找
       - 存储每一层的前驱节点到 `update` 数组
     - 随机决定新节点的层数
       - 使用 rng.gen_bool(0.5) 以 `50% 的概率` 递增层数
     - 如果新节点的层数超过最高 > 当前层数(new_level > self.level)
       - 把超出部分指向 `新的虚拟头节点`, 即为 None, 因为它们没有任何元素
         - 假设 level = 3，现在插入一个新节点，随机生成了 new_level = 5，那么 update 之前的情况
           - Level 5: head -> None
           - Level 4: head -> None
           - Level 3: head -> A -> B -> None
           - Level 2: head -> A -> B -> C -> None
           - Level 1: head -> A -> B -> C -> D -> None
         - 由于跳表目前的 level = 3，那么 level 4 和 level 5 之前都是 None
           - update[4] 和 update[3] 必须指向 head，因为它们没有任何元素
           - update[4] -> head
           - update[3] -> head
         - 总结：高于 level 的层数，head 是唯一的前驱节点，所以 update[i] 必须指向 head
       - 设置当前层数等于新节点的层数, self.level = new_level
      - 创建新节点
        - 确保 `head` 的 `forwards` 长度至少为 `new_level`
      - 从高层开始插入, 设置 `forwards` 指针
        - 设置当前层新节点的下一个节点指针 = 当前层的前驱节点下一个节点指针
        - 设置前驱节点的下一个节点指针 = 新节点

      例:
       在 `update` 存储的前驱节点之后插入新值
        - 假设有如下跳表
          - Level 3: head -> 20 -------------> None
          - Level 2: head -> 10 -> 20 -----> 30 -> None
          - Level 1: head -> 10 -> 20 -> 30 -> None
          - update 数组的查找过程
            目标值 15，从 最高层 Level 3 开始查找
            - Level 3（最高层）
              - head 的 forwards[2] 指向 20，但 20 > 15，所以 停在 head，update[2] = head
            - Level 2
              - head 的 forwards[1] 指向 10，10 < 15，所以向前走到 10
              - 10 的 forwards[1] 指向 20，但 20 > 15，所以 停在 10，update[1] = 10
            - Level 1
              - head 的 forwards[0] 指向 10，10 < 15，继续前进
              - 10 的 forwards[0] 指向 20，但 20 > 15，所以 停在 10，update[0] = 10
            - 最终的 update 数组
              - update[2] = head  (Level 3)
              - update[1] = 10    (Level 2)
              - update[0] = 10    (Level 1)
           - 在 update 存储的前驱节点后面插入 15
             - 假设 15 的层数 new_level = 2（通过 rng.gen_bool(0.5) 随机生成）
             - 插入 15 的过程
               - Level 1
                 - 15.forwards[0] = update[0].forwards[0]（即 15 连接 20）
                 - update[0].forwards[0] = 15（10 连接 15）
                - Level 2
                  - 15.forwards[1] = update[1].forwards[1]（15 连接 20）
                  - update[1].forwards[1] = 15（10 连接 15）
            - 插入后跳表
              - Level 3: head -> 20 -------------> None
              - Level 2: head -> 10 -> 15 -> 20 -> 30 -> None
              - Level 1: head -> 10 -> 15 -> 20 -> 30 -> None

       解释:
        - forwards:
          - forwards[i] 代表 当前节点在 i 层级的下一个节点
          - forwards 的大小取决于 当前节点的层级，例如某个节点的层级是 3，那它的 forwards 就有 3 个元素（索引 0、1、2）
          - head 节点的 forwards 长度一般是 MAX_LEVEL（最大允许层数），即便当前跳表的 level 没那么高，它的 forwards 仍然是最大的，确保可以动态扩展层数
          - 我们插入 3 个数字：10, 20, 30，并生成如下跳表：
            - Level 3: head -> 20 -------------> None
            - Level 2: head -> 10 -> 20 -----> 30 -> None
            - Level 1: head -> 10 -> 20 -> 30 -> None
            - head 的 forwards:
              - head.forwards[0] 指向 10
              - head.forwards[1] 指向 10
              - head.forwards[2] 指向 20
              - head.forwards[3] 指向 20  // head.forwards[3] 也指向 20，因为 head 在 Level 4 以上没有指向更高的节点
            - 10 的 forwards:
              - 10.forwards[0] 指向 20
              - 10.forwards[1] 指向 20
              - 10.forwards[2] 指向 None
              - 10.forwards[3] 指向 None // 10.forwards[3] 也指向 None，因为 10 在 Level 4 以上没有指向更高的节点
            - 20 的 forwards:
              - 20.forwards[0] 指向 30
              - 20.forwards[1] 指向 30
              - 20.forwards[2] 指向 None
              - 20.forwards[3] 指向 None // 20.forwards[3] 也指向 None，因为 20 在 Level 4 以上没有指向更高的节点

*/

use rand::Rng;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

// 跳表最大层数
const MAX_LEVEL: usize = 4;

// 每一层最大节点数
const PER_MAX_LEVEL: usize = 5;

#[derive(Debug)]
pub struct Node<T> {
    value: Option<T>,
    forwards: Vec<Option<Rc<RefCell<Node<T>>>>>, // 存储 `指向不同层的下一个节点` 的指针(多层指针)
}

impl<T> Node<T> {
    fn new(value: Option<T>, level: usize) -> Self {
        Self {
            value,
            forwards: vec![None; level],
        }
    }

    fn create(value: Option<T>, level: usize) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Self::new(value, level)))
    }
}

pub struct Skip<T> {
    head: Rc<RefCell<Node<T>>>, // 虚拟头节点, 不存储数据
    level: usize,               // 当前跳表最大层数
    max_level: usize,           // 最大层数
    size: usize,                // 当前节点数量
}

impl<T: Debug + Clone + Ord> Skip<T> {
    pub fn new(level: Option<usize>) -> Self {
        let max_level = level.unwrap_or(MAX_LEVEL);
        Self {
            head: Node::create(None, max_level),
            level: 1,
            max_level,
            size: 0,
        }
    }

    // 插入
    pub fn add(&mut self, value: T) {
        // 1. 定义 `update` 数组, 用于存储 `每一层的前驱节点`
        let mut update = vec![None; self.max_level];
        let mut current = self.head.clone();

        // 从 1.1 `head` 开始，找到每一层的前驱节点, 从高层向低层查找
        for i in (0..self.level).rev() {
            while let Some(forward) = current
                .clone()
                .borrow()
                .forwards
                .get(i)
                .and_then(|f| f.as_ref())
            {
                if forward.borrow().value < Some(value.clone()) {
                    current = Rc::clone(&forward);
                } else {
                    break;
                }
            }

            // 1. 2 存储每一层的前驱节点到 `update` 数组
            // `update[i]` 代表第 `i + 1` 层的前驱节点，用于连接新插入的节点
            update[i] = Some(Rc::clone(&current));
        }

        // 2. 随机决定新节点的层数
        let new_level = self.random_level();

        // 3.1 如果新节点的层数超过最高 > 当前层数(new_level > self.level)
        if new_level > self.level {
            // 3.2 把超出部分指向 `新的虚拟头节点`, 即为 None, 因为它们没有任何元素
            for i in self.level..new_level {
                update[i] = Some(Rc::clone(&self.head));
            }

            // 3.3 设置当前层数等于新节点的层数, self.level = new_level
            self.level = new_level;
        }

        // 4. 创建新节点
        let new_node = Node::create(Some(value), self.max_level);

        // 5. 从高层开始插入, 设置 `forwards` 指针
        for i in 0..new_level {
            if let Some(prev) = &update[i] {
                // 5.1 设置当前层新节点的下一个节点指针 = 当前层的前驱节点下一个节点指针
                new_node.borrow_mut().forwards[i] = prev.borrow_mut().forwards[i].take();

                // 5.2 设置前驱节点的下一个节点指针 = 新节点
                prev.borrow_mut().forwards[i] = Some(Rc::clone(&new_node));
            }
        }

        self.size += 1;
    }

    // 查找
    pub fn get(&self, value: T) -> bool {
        let mut current = self.head.clone();

        // 从高层开始查找
        for i in (0..self.level).rev() {
            while let Some(forward) = &current.clone().borrow().forwards[i] {
                if forward.borrow().value < Some(value.clone()) {
                    current = Rc::clone(&forward);
                } else {
                    break;
                }
            }
        }

        if let Some(next) = &current.borrow().forwards[0] {
            return next.borrow().value == Some(value);
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
        for i in (0..self.level).rev() {
            while let Some(forward) = current
                .clone()
                .borrow()
                .forwards
                .get(i)
                .and_then(|f| f.as_ref())
            {
                if forward.borrow().value < Some(value.clone()) {
                    current = Rc::clone(&forward);
                } else {
                    break;
                }
            }

            // 1. 2 存储每一层的前驱节点到 `update` 数组
            // `update[i]` 代表第 `i + 1` 层的前驱节点，用于连接新插入的节点
            update[i] = Some(Rc::clone(&current));
        }

        let next = current.borrow().forwards[0].clone();
        if let Some(next) = next {
            if next.borrow().value != Some(value.clone()) {
                return false;
            }

            found = true;

            // 从高层开始更新 forward 指针, 删除节点
            for i in (0..self.level).rev() {
                if let Some(prev) = &update[i] {
                    let next_forward = prev.borrow().forwards[i].clone();
                    if let Some(forward) = next_forward.as_ref() {
                        if forward.borrow().value == Some(value.clone()) {
                            prev.borrow_mut().forwards[i] = next.borrow_mut().forwards[i].take();
                        }
                    }
                }

                // 更新跳表层数, 确保跳表不会有冗余的空层
                // 检查跳表的最高层（level）是否有多余的空层，如果有，就会逐渐减少跳表的层数，直到没有空的层为止。
                while self.level > 1 && self.head.borrow().forwards[i].is_none() {
                    self.level -= 1;
                }
            }
        }

        if found {
            self.size -= 1;
        }

        found
    }

    // 先删除旧值，再插入新值
    pub fn update(&mut self, value: T, new_value: T) -> bool {
        if self.remove(value) {
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
        for i in (0..self.level).rev() {
            print!("Level {}: head", i + 1);
            let mut current = Rc::clone(&self.head);
            while let Some(next) = current
                .clone()
                .borrow()
                .forwards
                .get(i)
                .and_then(|f| f.as_ref())
            {
                print!(" -> {:?}", next.borrow().value);
                current = Rc::clone(&next);
            }

            print!(" -> None");
            println!()
        }
    }

    #[allow(dead_code)]
    pub fn print_update(&self, update: &Vec<Option<Rc<RefCell<Node<T>>>>>) {
        println!("update:");
        for i in 0..update.len() {
            print!("Update {}: ", i + 1);
            let mut current = Rc::clone(&self.head);
            while let Some(next) = current
                .clone()
                .borrow()
                .forwards
                .get(i)
                .and_then(|f| f.as_ref())
            {
                print!(" -> {:?}", next.borrow().value);
                current = Rc::clone(&next);
            }

            print!(" -> None");
            println!()
        }
    }
}
