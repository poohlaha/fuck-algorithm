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
