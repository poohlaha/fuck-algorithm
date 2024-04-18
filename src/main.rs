use crate::link::list::{merge_two_list, partition, print};
use crate::tree::binary_heap::{BinaryHeap, BinaryMaxHeap, BinaryMinHeap};

mod error;
mod link;
mod tree;

fn main() {
    test_merge_two_list();
    test_partition();
    test_min_binary_heap();
    test_max_binary_heap();
}

/// 测试 `合并两个有序链表`
fn test_merge_two_list() {
    let head = merge_two_list(vec![1, 3, 5, 7], vec![0, 2, 4, 6, 8, 9]);
    print(head);
}

/// 测试 `分隔链表`
fn test_partition() {
    println!("{}", "");
    let head = partition(vec![1, 4, 3, 2, 5, 2], 3);
    print(head);
}

/// 测试最小二叉堆
fn test_min_binary_heap() {
    println!("{}", "");
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
}

/// 测试最大二叉堆
fn test_max_binary_heap() {
    println!("{}", "");
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
}
