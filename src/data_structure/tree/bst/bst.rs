/*!
    二叉搜索树(Binary Search Tree，简称 BST)
    - 特点
     - 每个节点的左子树的所有节点都比当前节点小, 所以查找 `最小节点` 从左子树上找
     - 右子树的所有节点都比当前节点大, 所以查找 `最大节点` 从右子树上找
    - 定义
      - 存储节点的数据: data
      - 指向左子节点的指针: left
      - 指向右子节点的指针: right
*/
use std::collections::VecDeque;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<T> {
    data: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Clone + Debug + Display + PartialOrd> TreeNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    fn create(data: T) -> Box<TreeNode<T>> {
        Box::new(Self::new(data))
    }

    // 插入
    pub fn insert(&mut self, data: T) {
        if data < self.data {
            // 插入左子树
            if let Some(ref mut left) = self.left {
                left.insert(data);
            } else {
                self.left = Some(Self::create(data));
            }
        } else {
            // 插入右子树
            if let Some(ref mut right) = self.right {
                right.insert(data);
            } else {
                self.right = Some(Self::create(data));
            }
        }
    }

    // 删除
    // 1. 删除的节点没有子节点: 直接删除该节点
    // 2. 删除的节点只有一个子节点: 用该子节点替换删除的节点
    // 3. 删除的节点有两子节点: 找到右子树的最小值或左子树的最大值来替代当前节点的值, 然后递归删除替代的节点, 通常选择右子树的最小值
    pub fn remove(&mut self, data: T) -> Option<Box<TreeNode<T>>> {
        if data < self.data {
            // 在左子树删除
            if let Some(ref mut left) = self.left {
                self.left = left.remove(data);
            }
        } else if data > self.data {
            // 在右子树删除
            if let Some(ref mut right) = self.right {
                self.right = right.remove(data);
            }
        } else {
            // 已找到要删除的节点
            // 1. 删除的节点只有一个子节点: 用该子节点替换删除的节点
            if self.left.is_none() {
                return self.right.take();
            } else if self.right.is_none() {
                return self.left.take();
            } else {
                // 2. 删除的节点有两子节点: 找到右子树的最小值(中序后继)或左子树的最大值(中序前驱)来替代当前节点的值, 然后递归删除替代的节点
                let mut right = self.right.take().unwrap();
                // 找到右子树的最小节点
                let right_min_node = right.find_right_min().data.clone();
                // 替换当前节点的数据
                self.data = right_min_node;

                // 删除右子树中的最小节点, 此时替代节点出现了两次
                self.right = right.remove(self.data.clone());
            }
        }

        Some(Box::new(self.clone()))
    }

    // 找到左子树的最大值, 左子树的最大值就是它的最右叶子节点
    fn find_left_max(&self) -> &TreeNode<T> {
        match &self.right {
            None => self,
            Some(right) => right.find_left_max(),
        }
    }

    // 找到右子树的最小值, 右子树的最小值就是它的最左叶子节点
    fn find_right_min(&self) -> &TreeNode<T> {
        match &self.left {
            None => self,
            Some(left) => left.find_right_min(),
        }
    }

    // 查找
    pub fn find(&self, data: T) -> Option<&TreeNode<T>> {
        if data == self.data {
            return Some(self);
        }

        if data < self.data {
            if let Some(ref left) = self.left {
                left.find(data)
            } else {
                None
            }
        } else {
            if let Some(ref right) = self.right {
                right.find(data)
            } else {
                None
            }
        }
    }

    /*
                 50            - 层 1
                /  \
              30    70         - 层 2
             /  \   /  \
            20   40 60   80    - 层 3
    */
    // 前序遍历, 从 父节点 -> 左节点 -> 右节点 遍历
    // 1. 前序遍历：50 → 30 → 20 → 40 → 70 → 60 → 80    根节点 → 左子树 → 右子树
    pub fn preorder(&self, node: &Option<Box<TreeNode<T>>>) {
        if let Some(node) = node {
            println!("{}", node.data);
            self.preorder(&node.left);
            self.preorder(&node.right);
        }
    }

    // 中序遍历, 从 左节点 -> 父节点 -> 右节点, 左子树 -> 右子树 遍历
    // 2. 中序遍历：20 → 30 → 40 → 50 → 60 → 70 → 80    左子树 → 根节点 → 右子树
    pub fn inorder(&self, node: &Option<Box<TreeNode<T>>>) {
        if let Some(node) = node {
            self.inorder(&node.left);
            println!("{}", node.data);
            self.inorder(&node.right);
        }
    }

    // 后序遍历, 从 左节点 -> 右节点 -> 父节点 遍历
    // 3. 后序遍历：20 → 40 → 30 → 60 → 80 → 70 → 50    左子树(左子节点在最后) → 右子树(右子节点在最后) → 根节点
    pub fn postorder(&self, node: &Option<Box<TreeNode<T>>>) {
        if let Some(node) = node {
            self.postorder(&node.left);
            self.postorder(&node.right);
            println!("{}", node.data);
        }
    }

    // 层序遍历, 使用队列
    // 4. 层序遍历：50 → 30 → 70 → 20 → 40 → 60 → 80    逐层访问树中的节点
    pub fn level_order(&self) {
        let mut queue = VecDeque::new(); // 使用 VecDeque 作为队列
        queue.push_back(Some(Box::new(self.clone()))); // 将根节点加入队列

        while let Some(node) = queue.pop_front() {
            if let Some(node) = node {
                println!("{:?}", node.data);

                // 将左子节点加入队列
                queue.push_back(node.left.clone());

                // 将右子节点加入队列
                queue.push_back(node.right.clone());
            }
        }
    }

    // 通过数据创建树
    pub fn create_tree(&mut self, arr: Vec<T>) {
        if arr.is_empty() {
            return;
        }

        for i in arr.iter() {
            self.insert(i.clone());
        }
    }

    // 树的深度
    pub fn depth(&self) -> usize {
        let left_depth = self.left.as_ref().map_or(0, |node| node.depth());
        let right_depth = self.right.as_ref().map_or(0, |node| node.depth());
        std::cmp::max(left_depth, right_depth) + 1
    }

    pub fn print(&self) {
        let mut queue = VecDeque::new();
        queue.push_back((Some(self), 0));
        let max_depth = self.depth();

        while queue.len() > 0 {
            let level_size = queue.len();
            let mut current_level = vec![];

            for _ in 0..level_size {
                if let Some((node, level)) = queue.pop_front() {
                    if let Some(node) = node {
                        current_level.push((Some(node.data.clone()), level));

                        if node.left.is_none() && node.right.is_none() {
                            continue;
                        }

                        if let Some(ref left) = node.left {
                            queue.push_back((Some(left), level + 1));
                        } else {
                            queue.push_back((None, level + 1));
                        }

                        if let Some(ref right) = node.right {
                            queue.push_back((Some(right), level + 1));
                        } else {
                            queue.push_back((None, level + 1));
                        }
                    } else {
                        if level > 0 {
                            current_level.push((None, level));
                        }
                    }
                }
            }

            // println!("{:#?}", current_level);
            let mut spec = String::new();
            let mut value = String::new();
            for (i, (data, level)) in current_level.iter().enumerate() {
                let indent = " ".repeat((max_depth - level) * 4);
                if level.clone() == 0 {
                    println!("{}{}", indent, data.clone().unwrap());
                    continue;
                }

                if i % 2 == 0 {
                    spec += &format!("{}/", indent);
                } else {
                    if data.is_none() {
                        spec += &format!("{}{}", indent, indent);
                    } else {
                        spec += &format!("{}\\", indent);
                    }
                }

                if let Some(v) = data.clone() {
                    value += &format!("{}{}", indent, v);
                } else {
                    value += &format!("{}{}", indent, indent);
                }
            }

            if !spec.is_empty() {
                println!("{}", spec);
            }

            if !value.is_empty() {
                println!("{}", value);
            }
        }
    }
}
