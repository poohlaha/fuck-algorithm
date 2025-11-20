/*!
  累加树(BST to Greater Tree)
*/

use std::cell::RefCell;
use std::rc::Rc;

pub struct Greater;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/**
   538. 把二叉搜索树转换为累加树
   力扣: https://leetcode.cn/problems/convert-bst-to-greater-tree/description/
   题目: 给出二叉 搜索 树的根节点，该树的节点值各不相同，请你将其转换为累加树（Greater Sum Tree），使每个节点 node 的新值等于原树中大于或等于 node.val 的值之和。
        提醒一下，二叉搜索树满足下列约束条件:
        节点的左子树仅包含键 小于 节点键的节点。
        节点的右子树仅包含键 大于 节点键的节点。
        左右子树也必须是二叉搜索树。
*/
impl Greater {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let mut sum = 0;
        Self::reverse_inorder(root.clone(), &mut sum);
        root
    }

    // 反中序遍历: 右 → 根 → 左
    fn reverse_inorder(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(node_rc) = root {
            let node = node_rc.clone();
            let mut n = node.borrow_mut();

            // 先访问右子树
            Self::reverse_inorder(n.right.clone(), sum);

            // 累加 sum 并更新当前节点
            *sum += n.val;
            n.val = *sum;

            // 再访问左子树
            Self::reverse_inorder(n.left.clone(), sum);
        }
    }
}
