/*!
  B+ 树, 默认为偶数阶,  m = 2t => t = m / 2 => 2t - 1 = m - 1
*/

use std::fmt::Debug;
use std::sync::{Arc, Mutex};

// 阶数 m：每个节点最多有 m - 1 个键，m 个子节点
const MIN_DEGREE: usize = 2; // 即 t，最小度数，表示每个非根节点至少有 t-1 个键

#[derive(Clone, Debug, Default)]
pub struct TreeNode<T> {
    data: Option<T>,                        // 叶子节点存储数据
    key: i32,                               // 节点唯一标识符（可作为节点编号）
    is_leaf: bool, // 是否是叶子节点, true: 当前节点是叶子节点，即没有任何子节点，children 数组应为空。数据都存储在叶子节点中, false: 当前节点是内部节点，有子节点。它只存储索引（keys）和指向子节点的引用
    keys: Vec<i32>, // 子节点的 key(用于搜索)
    children: Vec<Arc<Mutex<TreeNode<T>>>>, // 子节点引用
    // 叶子节点链表指针
    prev: Option<Arc<Mutex<TreeNode<T>>>>, // 下一个叶子节点
    next: Option<Arc<Mutex<TreeNode<T>>>>, // 上一个叶子节点
}

impl<T: Clone + PartialEq> TreeNode<T> {
    pub fn new(
        key: i32,
        data: Option<T>,
        is_leaf: bool,
        keys: Vec<i32>,
        children: Vec<Arc<Mutex<TreeNode<T>>>>,
        prev: Option<Arc<Mutex<TreeNode<T>>>>,
        next: Option<Arc<Mutex<TreeNode<T>>>>,
    ) -> Self {
        Self {
            data,
            is_leaf,
            key,
            keys,
            children,
            prev,
            next,
        }
    }

    pub fn create(
        key: i32,
        data: Option<T>,
        is_leaf: bool,
        keys: Vec<i32>,
        children: Vec<Arc<Mutex<TreeNode<T>>>>,
        prev: Option<Arc<Mutex<TreeNode<T>>>>,
        next: Option<Arc<Mutex<TreeNode<T>>>>,
    ) -> Arc<Mutex<TreeNode<T>>> {
        Arc::new(Mutex::new(TreeNode::new(
            key, data, is_leaf, keys, children, prev, next,
        )))
    }
}

pub struct BPlusTree<T> {
    root: Arc<Mutex<TreeNode<T>>>,
    t: usize,           // 最小度数 (每个节点最少 t-1 个 key，最多 2t-1 个 key)
    current_index: i32, // 唯一节点编号生成器
}

impl<T: Clone + Debug + PartialEq + Ord> BPlusTree<T> {
    pub fn new(t: Option<usize>) -> Self {
        let size = if let Some(t) = t { t } else { MIN_DEGREE };

        let root = TreeNode::create(0, None, true, Vec::new(), Vec::new(), None, None);
        Self {
            root,
            t: size,
            current_index: 1, // 从 1 开始
        }
    }

    // 插入, 需要判断是不是满
    pub fn insert(&mut self, data: T) {
        let root_full = {
            let guard = self.root.lock().unwrap();
            let keys = guard.keys.clone();
            keys.len() == 2 * self.t - 1
        };

        // 1. 根节点未满, 直接调用 `3. 非满插入`
        if !root_full {
            // 1.1 直接调用 `3. 非满插入`
            self.insert_non_full(None, data);
        } else {
            // 2. 根节点已满(len(node.keys) == 2t - 1)
            // 2.1 此时需要分裂 `root`, 调用 `4. 分裂`, 返回 `new_node`
            // 根节点不需要 设置 prev 和 next
            let (mid_key, new_node) = self.split(None);
            let old_root = self.root.clone();
            let new_root = TreeNode::create(
                self.current_index,
                None,
                false,
                vec![mid_key],
                vec![old_root.clone(), new_node],
                None,
                None,
            );
            self.current_index += 1;

            // 2.2 `new_root` = `root`
            self.root = new_root;

            // 2.3 重新锁定新的根节点
            // 2.3 调用 `3. 非满插入`
            self.insert_non_full(None, data);
        }
    }

    // 3. 非满插入
    fn insert_non_full(&mut self, root_node: Option<Arc<Mutex<TreeNode<T>>>>, data: T) {
        let key = self.current_index;
        self.current_index += 1;

        let node = if let Some(root_node) = root_node {
            root_node
        } else {
            self.root.clone()
        };

        let mut node = node.lock().unwrap();

        // 3.1 叶子节点(此时没有子节点, 直接插入即可)
        if node.is_leaf {
            // 在正确的位置插入 key (而不是 push)，可用二分查找
            let pos = node.keys.binary_search(&key).unwrap_or_else(|e| e);
            node.keys.insert(pos, key);
            node.data = Some(data);
            node.key = self.current_index.clone();
        } else {
            // 3.2 非叶子节点
            // 3.2.1 通过 `key < keys[i]`, 找到合适的子节点 `children[i]`(keys 升序, data 必定存在 children 中)
            // 使用 二分法 找到合适的子节点索引 i
            let mut i = node.keys.len();
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }

            // 3.2.2 判断节点是否已满
            let child = node.children[i].clone();
            let child = child.lock().unwrap();
            if child.keys.len() == 2 * self.t - 1 {
                // 调用 `4. 分裂`
                let (mid_key, new_root) = self.split(Some(node.children[i].clone()));

                // 3.2.3 把 `中间的 key` 移到 `父节点`
                node.keys.insert(i, mid_key.clone());

                // 3.2.4 把新节点插入到父节点对应位置
                node.children.insert(i + 1, new_root.clone());

                // 重新获取插入位置 `i`, 插入到分裂后的部分(需要判断左或右半部分)
                if key > mid_key {
                    i += 1
                }

                drop(child);

                // 3.2.4 重新锁定 child 节点
                let child = node.children[i].clone();

                // 调用 `3. 非满插入`
                self.insert_non_full(Some(child), data);
            } else {
                // 调用 `3. 非满插入`
                self.insert_non_full(Some(node.children[i].clone()), data);
            }
        }
    }

    // 4. 分裂
    fn split(&mut self, node: Option<Arc<Mutex<TreeNode<T>>>>) -> (i32, Arc<Mutex<TreeNode<T>>>) {
        let node = if let Some(node) = node {
            node
        } else {
            self.root.clone()
        };

        let mut node = node.lock().unwrap();

        // 4.1 取 `中间的 key`(mid = t - 1)
        let keys = node.keys.clone();
        let mid = self.t - 1;
        let mid_key = keys[mid].clone();

        // 4.2 分裂 `节点 node 的 children`
        // 4.2.3 rightChildren = node[t.. len]
        // split_off 保留 前 N 个元素，返回后面的部分
        let right_children = if !node.is_leaf {
            node.children.split_off(mid + 1)
        } else {
            Vec::new()
        };

        // 4.2.2 leftChildren = node[0.. t - 1], node.children 现在是左半部分 [0..mid+1]
        // let left_children = node.children.clone();

        // 4.3 创建新节点 `new_node`, `新节点` 拥有 `右半部分`
        let mut new_node: TreeNode<T> = TreeNode::new(
            self.current_index,
            None,
            node.is_leaf,
            Vec::new(),
            Vec::new(),
            None,
            None,
        );

        // 4.3.1 new_node.keys = node.keys[mid + 1 .. len] (右半部分 keys )
        new_node.keys = node.keys.split_off(mid + 1);
        // 4.3.2 new_node.children = rightChildren
        new_node.children = right_children;

        self.current_index += 1;

        // 4.4 截取 node.keys 的左半部分留给原节点 (树的有序性)
        // 4.4.1 node.keys = node.keys[mid]
        // truncate 保留 前 N 个元素
        node.keys.truncate(mid);

        // 4.4.2 node.children = left_children, 不需要设置了, node.children.split_off(self.t) 自动获取前半部分
        // node.children = left_children;
        if node.is_leaf {
            let node_arc = Arc::new(Mutex::new(node.clone()));

            new_node.next = node.next.take();
            new_node.prev = Some(Arc::clone(&node_arc));

            if let Some(ref next_node) = new_node.next {
                let mut next_guard = next_node.lock().unwrap();
                next_guard.prev = Some(Arc::new(Mutex::new(new_node.clone())));
            }

            node.next = Some(Arc::new(Mutex::new(new_node.clone())));
        }

        // 维护双向链表
        (mid_key, Arc::new(Mutex::new(new_node)))
    }

    // 查找节点
    pub fn search(&self, data: T) -> Option<(Option<T>, i32)> {
        self.search_node(None, data)
    }

    fn search_node(
        &self,
        node: Option<Arc<Mutex<TreeNode<T>>>>,
        data: T,
    ) -> Option<(Option<T>, i32)> {
        let node = if let Some(node) = node {
            node
        } else {
            self.root.clone()
        };

        let node = node.lock().unwrap();

        // 1. 如果是叶子节点，直接在 data 中查找
        if node.is_leaf {
            if let Some(ref d) = node.data {
                if d == &data {
                    return Some((node.data.clone(), node.key));
                }
            }

            return None;
        }

        // 2. 非叶子节点，遍历所有子节点递归查找
        for child in node.children.clone() {
            if let Some((d, key)) = self.search_node(Some(child), data.clone()) {
                return Some((d, key));
            }
        }

        None
    }

    // 范围查找, 使用双向链表
    pub fn range_search(&self, start: i32, end: i32) -> Vec<(Option<T>, i32)> {
        let mut results = Vec::new();
        // 1. 先找到起始叶子节点
        let start_node = self.find_leaf_node(start);
        if start_node.is_none() {
            return results;
        }

        let mut current_node = start_node.unwrap();
        loop {
            let node_guard = current_node.lock().unwrap();
            // 遍历叶子节点里的 keys 和 data，找范围内的项
            for (_, key) in node_guard.keys.iter().enumerate() {
                if *key >= start && *key <= end {
                    results.push((node_guard.data.clone(), node_guard.key.clone()));
                } else if *key > end {
                    return results;
                }
            }

            // 2. 顺着 next 指针走到下一个叶子节点
            let next = node_guard.next.clone();
            drop(node_guard);

            if let Some(ref next_node) = next {
                current_node = next_node.clone();
            } else {
                break; // 到链表尾部了
            }
        }
        results
    }

    fn find_leaf_node(&self, key: i32) -> Option<Arc<Mutex<TreeNode<T>>>> {
        let mut node = self.root.clone();

        loop {
            let node_guard = node.lock().unwrap();
            if node_guard.is_leaf {
                return Some(node.clone());
            }

            // 找到对应的子节点索引
            let mut i = 0;
            while i < node_guard.keys.len() && key >= node_guard.keys[i] {
                i += 1;
            }

            let children = node_guard.children.clone();
            drop(node_guard);

            if i < children.len() {
                node = children[i].clone();
            } else {
                return None;
            }
        }
    }

    pub fn update(&mut self, data: T, new_data: T) -> bool {
        self.update_node(None, data, new_data)
    }

    fn update_node(&mut self, node: Option<Arc<Mutex<TreeNode<T>>>>, data: T, new_data: T) -> bool {
        let node = if let Some(node) = node {
            node
        } else {
            self.root.clone()
        };

        let mut node = node.lock().unwrap();

        // 1. 如果是叶子节点，直接在 data 中查找
        if node.is_leaf {
            if let Some(ref mut d) = node.data {
                if d == &data {
                    *d = new_data;
                    return true;
                }
            }

            return false;
        }

        // 2. 非叶子节点，遍历所有子节点递归
        for child in &node.children {
            if self.update_node(Some(child.clone()), data.clone(), new_data.clone()) {
                return true;
            }
        }

        false
    }

    // 删除, 删除 + 借位 / 合并 / 修复
    pub fn delete(&mut self, data: T) -> bool {
        let flag = self.delete_node(None, data);
        if flag {
            self.update_keys(None);
        }

        flag
    }

    fn delete_node(&mut self, node: Option<Arc<Mutex<TreeNode<T>>>>, data: T) -> bool {
        let node = if let Some(node) = node {
            node
        } else {
            self.root.clone()
        };

        let mut node = node.lock().unwrap();

        // 1. 如果是叶子节点，直接删除 data
        if node.is_leaf {
            if let Some(ref mut d) = node.data {
                if d == &data {
                    node.data = None;
                    return true;
                }
            }

            return false;
        }

        // 2. 非叶子节点，遍历所有子节点递归
        for child in &node.children {
            if self.delete_node(Some(child.clone()), data.clone()) {
                return true;
            }
        }

        false
    }

    // 更新非叶子节点的 keys
    fn update_keys(&mut self, node: Option<Arc<Mutex<TreeNode<T>>>>) {
        let node = if let Some(node) = node {
            node
        } else {
            self.root.clone()
        };

        let mut node = node.lock().unwrap();
        if node.is_leaf {
            return;
        }

        node.keys.clear();

        for child in node.children.clone() {
            let child_guard = child.lock().unwrap();
            node.keys.push(child_guard.key);
        }

        // 递归更新子节点
        for child in &node.children {
            self.update_keys(Some(child.clone()));
        }
    }
}
