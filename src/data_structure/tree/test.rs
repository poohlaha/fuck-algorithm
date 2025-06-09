use crate::data_structure::tree::bst::bst;
use crate::data_structure::tree::other::huff::HuffmanTree;
use std::collections::HashMap;
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

/// 测试 `霍夫曼树`
pub fn test_huffman_tree() {
    println!("----- Huffman Tree start ------");
    let mut frequency_map = HashMap::new();
    frequency_map.insert('A', 5);
    frequency_map.insert('B', 9);
    frequency_map.insert('C', 12);
    frequency_map.insert('D', 13);
    frequency_map.insert('E', 16);

    let mut tree = HuffmanTree::new(&frequency_map);

    println!("初始编码表: {:?}", tree.code_map);

    println!("\n🌳 初始树结构:");
    tree.print_tree();

    // 编码/解码
    let encoded_str = format!(
        "{}{}{}",
        tree.get_code('A').unwrap(),
        tree.get_code('B').unwrap(),
        tree.get_code('C').unwrap()
    );
    println!(
        "\n解码二进制串 \"{}\" ➜ {}",
        encoded_str,
        tree.decode(&encoded_str)
    );

    // 添加新符号
    tree.add_symbol(&mut frequency_map, 'F', 7);
    println!("\n添加 F 后编码表: {:?}", tree.code_map);
    tree.print_tree();

    // 修改 D 的频率
    tree.update_frequency(&mut frequency_map, 'D', 20);
    println!("\n修改 D 后编码表: {:?}", tree.code_map);
    tree.print_tree();

    // 删除 B
    tree.remove_symbol(&mut frequency_map, 'B');
    println!("\n删除 B 后编码表: {:?}", tree.code_map);
    tree.print_tree();

    // 带权路径
    let root = tree.get_tree_root();
    if let Some(root) = root {
        let wpl = tree.calculate_wpl(root, 0);
        println!("带权路径长度(WPL): {}", wpl);
    } else {
        println!("带权路径长度(WPL): {}", 0);
    }

    println!("----- Huffman Tree end ------");
}

pub fn test() {
    println!("----- tree start ------");
    test_bst();
    test_huffman_tree();
    println!("----- tree end ------");
}
