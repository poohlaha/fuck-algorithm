use crate::data_structure::tree::link;
use crate::data_structure::tree::order;

/*
      10
     /  \
    5    15
   / \   / \
  3   7 12  20
*/
/// 测试 `链式存储`
pub fn test_link_tree() {
    let mut tree = link::TreeNode::<i32>::new(10);
    println!("add:");
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(7);
    tree.insert(12);
    tree.insert(20);
    // tree.print();

    println!("remove:");
    tree.remove(5);
    tree.print();
}

/// 测试 `顺序存储`
/*
     10
    /  \
   5    15
  / \   / \
 3   7 12  20
*/
pub fn test_order_tree() {
    let mut tree = order::TreeNode::<i32>::new();
    println!("add:");
    tree.insert(10);
    tree.print();
    tree.insert(5);
    tree.print();
    tree.insert(20);
    tree.print();
    tree.insert(7);
    tree.print();
    tree.insert(15);
    tree.print();
    tree.insert(3);
    tree.print();
    tree.insert(12);
    tree.print(); // [10, 5, 15, 3, 7, 12, 20]

    println!("remove:");
    tree.remove(7);
    tree.print();
}

pub fn test() {
    println!("----- tree start ------");
    test_link_tree();
    // test_order_tree();
    println!("----- tree end ------");
}
