/*!
  AVL 树, 在多线程中读写
*/

use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct TreeNode<T> {
    data: T,
    height: i32,
    left: Option<Arc<Mutex<TreeNode<T>>>>,
    right: Option<Arc<Mutex<TreeNode<T>>>>,
}

impl<T: Ord> TreeNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            height: 1,
            left: None,
            right: None,
        }
    }

    pub fn create(data: T) -> Arc<Mutex<TreeNode<T>>> {
        Arc::new(Mutex::new(Self::new(data)))
    }
}

pub struct AvlTree<T> {
    root: Option<Arc<Mutex<TreeNode<T>>>>,
}

impl<T: Ord + Clone> AvlTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    // 插入
    pub fn insert(&mut self, data: T) {
        self.root = Self::tree_insert(self.root.take(), data)
    }

    fn tree_insert(
        node: Option<Arc<Mutex<TreeNode<T>>>>,
        data: T,
    ) -> Option<Arc<Mutex<TreeNode<T>>>> {
        if let Some(node) = node {
            let mut node = node.lock().unwrap();
            match data.cmp(&node.data) {
                Ordering::Less => {
                    node.left = Self::tree_insert(node.left.take(), data);
                }
                Ordering::Greater => {
                    node.right = Self::tree_insert(node.right.take(), data);
                }
                Ordering::Equal => return Some(TreeNode::create(data.clone())),
            }

            Some(Self::balance(node.clone()))
        } else {
            Some(TreeNode::create(data))
        }
    }

    // 获取高度
    fn height(node: &Option<Arc<Mutex<TreeNode<T>>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.lock().unwrap().height)
    }

    // 计算平衡因子: 平衡因子 = 左子树高度 - 右子树高度
    fn get_balance(node: &TreeNode<T>) -> i32 {
        Self::height(&node.left) - Self::height(&node.right)
    }

    // 检查当前节点的平衡因子, 并根据不同情况执行旋转操作, 确保 AVL 树的平衡
    // 计算平衡因子: 平衡因子 = 左子树高度 - 右子树高度
    // BF = 1、0、01: 表示该节点是平衡的(不需要旋转)
    // BF > 1: 表示左子树较高，可能需要 `右旋` 或 `先左后右旋(LR)`
    // BF < -1：表示右子树较高，可能需要 `左旋` 或 `先右后左旋(RL)`
    /*
     平衡调整策略
     1. BF > 1 (左子树过高)
        - 左左失衡(LL)：直接右旋
        - 左右失衡(LR)：先左旋, 再右旋
      2. BF < -1(右子树过高)
        - 右右失衡(RR)：直接左旋
        - 右左失衡(RL)：先右旋, 再左旋
    */
    fn balance(mut node: TreeNode<T>) -> Arc<Mutex<TreeNode<T>>> {
        let balance = Self::get_balance(&node);

        // BF > 1: 表示左子树较高，可能需要 `右旋` 或 `先左后右旋(LR)`
        if balance > 1 {
            // 左左失衡(LL)：直接右旋
            if Self::get_balance(&node.left.as_ref().unwrap().lock().unwrap()) >= 0 {
                return Self::rotate_right(Arc::new(Mutex::new(node)));
            } else {
                // 左右失衡(LR)：先左旋, 再右旋
                node.left = Some(Self::rotate_left(node.left.take().unwrap()));
                return Self::rotate_right(Arc::new(Mutex::new(node)));
            }
        }

        // BF < -1：表示右子树较高，可能需要 `左旋` 或 `先右后左旋(RL)`
        if balance < -1 {
            // 右右失衡(RR)：直接左旋
            if Self::get_balance(&node.right.as_ref().unwrap().lock().unwrap()) <= 0 {
                return Self::rotate_left(Arc::new(Mutex::new(node)));
            } else {
                // 右左失衡(RL)：先右旋, 再左旋
                node.right = Some(Self::rotate_right(node.right.take().unwrap()));
                return Self::rotate_left(Arc::new(Mutex::new(node)));
            }
        }

        Arc::new(Mutex::new(node))
    }

    // 左旋, 适用于 `右子树过高 (BF < -1) 的情况, 即 `RR失衡(右右失衡)`
    // 所有节点大小关系是： A < x < B < y < C
    /*
       x
      / \
     A   y         ===(左旋)==>          y
        / \                            / \
       B   C                          x   C
                                     / \
                                    A   B
    */
    fn rotate_left(node: Arc<Mutex<TreeNode<T>>>) -> Arc<Mutex<TreeNode<T>>> {
        let mut node = node.lock().unwrap();
        let right = node.right.take().unwrap(); // 取出右子树 y 及其子节点
        let mut right_locked = right.lock().unwrap();

        // 将 x 放到 y 的左边, 将 y 的左子树 B 变成 x 的 右节点, 根据平衡二叉树特性, 左子树的值 < 当前节点, 右子树的值 > 当前节点， 所以 B > x 的值, 插入到 x 右子树上
        node.right = right_locked.left.take();

        // 此时右子树 y 为根节点, y 的左节点为 x 节点
        right_locked.left = Some(Arc::new(Mutex::new(node.clone())));

        // 重新计算 x 节点的高度
        node.height = 1 + std::cmp::max(Self::height(&node.left), Self::height(&node.right));

        // 重新计算 y 节点的高度
        right_locked.height = 1 + std::cmp::max(
            Self::height(&right_locked.left),
            Self::height(&right_locked.right),
        );

        drop(node); // 提前释放对 x 的锁
        Arc::new(Mutex::new(right_locked.clone()))
    }

    // 右旋, 适用于 `左子树过高 (BF > 1) 的情况, 即 `LL失衡(左左失衡)`
    // 所有节点大小关系是： A < x < B < y < C
    /*
        y
       / \
      x   C         ===(右旋)==>            x
     / \                                  / \
    A   B                                A   y
                                            / \
                                           B   C
     */
    // 计算当前节点的高度
    // AVL 树的高度用于计算平衡因子: height= 1 + max(left subtree height,right subtree height)
    fn rotate_right(node: Arc<Mutex<TreeNode<T>>>) -> Arc<Mutex<TreeNode<T>>> {
        let mut node = node.lock().unwrap(); // y 节点
        let left = node.left.take().unwrap(); // 取出 x 及其子节点
        let mut left_locked = left.lock().unwrap();

        // 将 x 节点的右节点给 y 节点的左节点, 根据平衡二叉树特性, 左子树的值 < 当前节点, 右子树的值 > 当前节点， 所以 y > B 的值, 插入到 y 左子树上
        node.left = left_locked.right.take();

        // 将 x 的右子树 给 y 节点
        left_locked.right = Some(Arc::new(Mutex::new(node.clone())));

        // 重新计算 y 节点的高度
        node.height = 1 + std::cmp::max(Self::height(&node.left), Self::height(&node.right));

        // 重新计算根节点 x 的高度
        left_locked.height = 1 + std::cmp::max(
            Self::height(&left_locked.left),
            Self::height(&left_locked.right),
        );

        drop(node); // 释放平衡节点
        Arc::new(Mutex::new(left_locked.clone()))
    }
}
