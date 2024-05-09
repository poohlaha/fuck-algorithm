use crate::tree::binary_tree::{get_max_depth, get_max_depth2, TreeNode};

/// 测试 `合并两个有序链表`
fn test_get_max_depth() {
    let root = test_create_tree1();
    let deep = get_max_depth(root);
    println!("get max depth: {}", deep);
}

/// 测试 `查找二叉树的最大深度`
fn test_get_max_depth2() {
    let root = test_create_tree1();
    let deep = get_max_depth2(Some(Box::new(root)));
    println!("get max depth2: {}", deep);
}

/**
          3
        /   \
       9     20
     /     /    \
    4    15      7
**/
fn test_create_tree1() -> TreeNode<u32> {
    let mut root = TreeNode::new(3);
    let mut left = TreeNode::new(9);
    let mut right = TreeNode::new(20);
    left.left = Some(Box::new(TreeNode::new(4)));
    right.left = Some(Box::new(TreeNode::new(15)));
    right.right = Some(Box::new(TreeNode::new(7)));

    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));
    return root;
}

pub(crate) fn test() {
    println!("----- tree start ------");
    test_get_max_depth();
    test_get_max_depth2();
    println!("----- tree end ------");
    println!();
}
