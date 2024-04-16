use crate::link::list::{merge_two_list, partition, print};

mod link;
mod error;

fn main() {
   test_merge_two_list();
   test_partition();
}

/// 测试 `合并两个有序链表`
fn test_merge_two_list() {
    let head = merge_two_list(vec![1, 3, 5, 7], vec![0, 2, 4, 6, 8, 9]);
    print(head);
}

/// 测试 `分隔链表`
fn test_partition() {
    println!("{}", "");
    let head = partition(vec![1,4,3,2,5,2], 3);
    print(head);
}
