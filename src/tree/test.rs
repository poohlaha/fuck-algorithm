use crate::tree::binary_tree::{
    diameter_of_binary_tree, get_max_depth, get_max_depth2, get_node_count, get_pre_node_layer,
    inorder_traverse, level_traverse, permute, permute_new, postorder_traverse, preorder_traverse,
    solve_n_queens, TreeNode,
};

/// 测试 `合并两个有序链表`
fn test_get_max_depth() {
    let root = test_create_tree1();
    let deep = get_max_depth(root);
    println!("get max depth: {}\n", deep);
}

/// 测试 `查找二叉树的最大深度`
fn test_get_max_depth2() {
    let root = test_create_tree1();
    let deep = get_max_depth2(Some(Box::new(root)));
    println!("get max depth2: {}\n", deep);
}

/// 测试 `前序遍历`
fn test_preorder_traverse() {
    let root = test_create_tree2();
    let arr = preorder_traverse(Some(Box::new(root)));
    println!("preorder traverse: {:?}\n", arr);
}

/// 测试 `中序遍历`
fn test_inorder_traverse() {
    let root = test_create_tree2();
    let arr = inorder_traverse(Some(Box::new(root)));
    println!("inorder traverse: {:?}\n", arr);
}

/// 测试 `后序遍历`
fn test_postorder_traverse() {
    let root = test_create_tree2();
    let arr = postorder_traverse(Some(Box::new(root)));
    println!("postorder traverse: {:?}\n", arr);
}

/// 测试 `计算每个节点所在的层数`
fn test_get_pre_node_layer() {
    println!("get pre node layer ...");
    let root = test_create_tree2();
    get_pre_node_layer(Some(Box::new(root)), 1);
    println!("");
}

/// 测试 `计算每个节点的左右子树各有多少节点`
fn test_get_node_count() {
    println!("get node count ...");
    let root = test_create_tree2();
    get_node_count(Some(Box::new(root)));
    println!("");
}

/// 测试 `计算二叉树的最长直径长度`
fn test_diameter_of_binary_tree() {
    let root = test_create_tree3();
    let max = diameter_of_binary_tree(root);
    println!("diameter of binary tree: {}", max);
}

/// 测试 `层序遍历`
fn test_level_traverse() {
    let root = test_create_tree2();
    let arr = level_traverse(Some(Box::new(root)));
    println!("level traverse: {:?}", arr);
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

/**
          1
        /   \
      2       3
    /  \    /   \
   5    4  8    9
      /   \
     6    7

           root  ----- root.left -----    -- root.right--
  preorder: 1    2    5    4    6    7    3    8    9
  root: 1
  root.left: 2 5 4 6 7
  root.right 3 8 9
**/
fn test_create_tree2() -> TreeNode<u32> {
    let mut root = TreeNode::new(1);
    let mut node2 = TreeNode::new(2);
    let mut node3 = TreeNode::new(3);
    let mut node4 = TreeNode::new(4);
    let node5 = TreeNode::new(5);
    let node6 = TreeNode::new(6);
    let node7 = TreeNode::new(7);
    let node8 = TreeNode::new(8);
    let node9 = TreeNode::new(9);

    node3.left = Some(Box::new(node8));
    node3.right = Some(Box::new(node9));

    node4.left = Some(Box::new(node6));
    node4.right = Some(Box::new(node7));

    node2.left = Some(Box::new(node5));
    node2.right = Some(Box::new(node4));

    root.left = Some(Box::new(node2));
    root.right = Some(Box::new(node3));

    return root;
}

/**
          9
            \
             1
           /   \
          2    3
        /   \
       5     4
*/
fn test_create_tree3() -> TreeNode<u32> {
    let mut root = TreeNode::new(9);
    let mut node1 = TreeNode::new(1);
    let mut node2 = TreeNode::new(2);
    let node3 = TreeNode::new(3);
    let node4 = TreeNode::new(4);
    let node5 = TreeNode::new(5);

    node2.left = Some(Box::new(node5));
    node2.right = Some(Box::new(node4));

    node1.left = Some(Box::new(node2));
    node1.right = Some(Box::new(node3));
    root.right = Some(Box::new(node1));

    return root;
}

fn test_permute() {
    let nums = vec![1, 2, 3];
    let result = permute(nums);
    println!("permute: {:?}", result);
}

fn test_permute_new() {
    let nums = vec![1, 2, 3];
    let result = permute_new(nums);
    println!("permute new: {:?}", result);
}

fn test_solve_n_queens() {
    let result = solve_n_queens(2);
    println!("solve 8 queens: {:?}", result);
}

pub(crate) fn test() {
    println!("----- tree start ------");
    test_get_max_depth();
    test_get_max_depth2();
    test_preorder_traverse();
    test_inorder_traverse();
    test_postorder_traverse();
    test_get_pre_node_layer();
    test_get_node_count();
    test_diameter_of_binary_tree();
    test_level_traverse();
    test_permute();
    test_permute_new();
    test_solve_n_queens();
    println!("----- tree end ------");
    println!();
}
