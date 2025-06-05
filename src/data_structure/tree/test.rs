use crate::data_structure::tree::bst::bst;

/*
      10
     /  \
    5    15
   / \   / \
  3   7 12  20
*/
/// 测试 `BST`
pub fn test_bst() {
    let mut tree = bst::TreeNode::<i32>::new(10);
    println!("add:");
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(7);
    tree.insert(12);
    tree.insert(20);
    tree.print();

    println!("remove:");
    tree.remove(5);
    tree.print();
}

pub fn test() {
    println!("----- tree start ------");
    test_bst();
    println!("----- tree end ------");
}
