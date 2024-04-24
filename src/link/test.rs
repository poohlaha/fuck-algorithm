use crate::link::list::{
    create, detect_cycle, get_intersection_node, has_cycle, merge_k_list, merge_two_list,
    middle_node, partition, print, remove_n_from_end,
};
use crate::tree::binary_heap::{BinaryHeap, BinaryMaxHeap, BinaryMinHeap};

/// 测试 `合并两个有序链表`
fn test_merge_two_list() {
    println!("merge two list:");
    let head = merge_two_list(vec![1, 3, 5, 7], vec![0, 2, 4, 6, 8, 9]);
    print(head);
    println!("\n")
}

/// 测试 `分隔链表`
fn test_partition() {
    println!("partition:");
    let head = partition(vec![1, 4, 3, 2, 5, 2], 3);
    print(head);
    println!("\n")
}

/// 测试 `合并 K 个有序链表`
fn test_merge_k_list() {
    println!("merge k list");
    let head = merge_k_list(vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]]);
    print(head);
    println!("\n")
}

/// 测试 `删除链表的倒数第 N 个结点`
fn test_remove_n_from_end() {
    println!("remove n from end");
    let head = remove_n_from_end(vec![1, 4, 5, 7, 3, 4, 2, 6], 4);
    println!("{:#?}", head);
    println!("\n")
}

/// 测试 `删除链表的倒数第 N 个结点`
fn test_middle_node() {
    println!("middle node");
    let head = middle_node(vec![1, 4, 5, 7, 3, 4, 2, 6]);
    println!("{:#?}", head);
    println!("\n")
}

/// 测试 `判断链表是否包含环`
fn test_has_cycle() {
    println!("has cycle");
    let head = create(vec![1, 2, 5, 7, 3, 4, 2]);
    let flag = has_cycle(head);
    println!("{:#?}", flag);
    println!("\n")
}

/// 测试 `寻找环形链表起点`
fn test_detect_cycle() {
    println!("detect cycle");
    let head = create(vec![1, 2, 5, 7, 3, 4, 2]);
    let flag = detect_cycle(head);
    println!("{:#?}", flag);
    println!("\n")
}

/// 测试 `两个链表是否相交`
fn test_get_intersection_node() {
    println!("get intersection node");
    let head = get_intersection_node(vec![1, 2, 5, 7, 4], vec![3, 8, 1, 2]);
    println!("{:#?}", head);
    println!("\n")
}

/// 测试最小二叉堆
fn test_min_binary_heap() {
    println!("min binary heap");
    let mut heap = BinaryMinHeap::new();
    heap.push(5);
    heap.push(3);
    heap.push(7);
    heap.push(1);
    heap.push(9);
    heap.push(2);
    heap.push(4);
    heap.push(6);

    /*
               1
             /   \
            3     2
          /  \   / \
         5    9 7   4
        /
       6
    */

    println!("{:?}", heap);

    // delete
    println!("{:?}", heap.delete());
    println!("{:?}", heap);

    /*
          2
        /   \
       3     4
     /  \   / \
    5    9 7   6
    */

    // delete
    println!("{:?}", heap.delete());
    println!("{:?}", heap);

    /*
          3
        /   \
       5     4
     /  \   /
    6    9 7
     */

    println!("\n")
}

/// 测试最大二叉堆
fn test_max_binary_heap() {
    println!("max binary heap");
    let mut heap = BinaryMaxHeap::new();
    heap.push(5);
    heap.push(3);
    heap.push(7);
    heap.push(1);
    heap.push(9);
    heap.push(2);
    heap.push(4);
    heap.push(6);

    println!("{:?}", heap);

    /*
             9
           /   \
          7     5
        /  \   / \
       6    3 2   4
      /
     1
    */

    // delete
    println!("{:?}", heap.delete());
    println!("{:?}", heap);

    /*
          7
        /   \
       6     5
     /  \   / \
    1    3 2   4
    */

    // delete
    println!("{:?}", heap.delete());
    println!("{:?}", heap);

    /*
           6
         /   \
        4     5
      /  \   /
     1    3 2
    */

    println!("\n")
}

pub(crate) fn test() {
    test_merge_two_list();
    test_partition();
    test_merge_k_list();
    test_remove_n_from_end();
    test_middle_node();
    test_has_cycle();
    test_detect_cycle();
    test_get_intersection_node();
    test_min_binary_heap();
    test_max_binary_heap();
}
