use crate::leet::dbpointer::arr::remove_duplicates;
use crate::leet::dbpointer::integer::is_palindrome;
use crate::leet::dbpointer::link::{merge_k_lists, reverse_k_group, swap_pairs, Link};
use crate::leet::dbpointer::max_area;

/// 测试 `回文数`
fn test_is_palindrome() {
    println!("----- leet code palindrome start ------");
    let result = is_palindrome(121);
    println!("is palindrome: {:}", result);

    let result = is_palindrome(-121);
    println!("is palindrome: {:}", result);

    let result = is_palindrome(3);
    println!("is palindrome: {:}", result);

    let result = is_palindrome(10);
    println!("is palindrome: {:}", result);

    let result = is_palindrome(21120);
    println!("is palindrome: {:}", result);

    println!("----- leet code palindrome end ------");
}

/// 测试 `盛最多水的容器`
fn test_max_area() {
    println!("----- leet code max area start ------");
    let result = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("max area: {:}", result);

    let result = max_area(vec![1, 1]);
    println!("max area: {:}", result);
    println!("----- leet code max area end ------");
}

/// 测试 `删除链表的倒数第 N 个节点`
fn test_remove_nth_from_end() {
    println!("----- leet code remove link n node start ------");
    let mut head = Link::create_node(vec![1, 2, 3, 4, 5]);
    head = Link::remove_nth_from_end(head, 2);
    println!("head: {:#?}", head);
    println!("----- leet code remove link n node end ------");
}

/// 测试 `合并两个有序链表`
fn test_merge_two_lists() {
    println!("----- leet code merge two list start ------");
    let list1 = Link::create_node(vec![1, 2, 4]);
    let list2 = Link::create_node(vec![1, 3, 4, 5]);
    let result = Link::merge_two_lists(list1, list2);
    println!("head: {:#?}", result);
    println!("----- leet code merge two list end ------");
}

/// 测试 `合并 K 个升序链表`
fn test_merge_k_lists() {
    println!("----- leet code merge k list start ------");
    let list1 = Link::create_node(vec![1, 4, 5]);
    let list2 = Link::create_node(vec![1, 3, 4]);
    let list3 = Link::create_node(vec![2, 6]);
    let result = merge_k_lists(vec![list1, list2, list3]);
    println!("head: {:#?}", result);

    let result = merge_k_lists(vec![None]);
    println!("head: {:#?}", result);

    println!("----- leet code merge k list end ------");
}

/// 测试 `两两交换链表中的节点`
fn test_swap_pairs() {
    println!("----- leet code swap pairs start ------");
    let list1 = Link::create_node(vec![1, 2, 3, 4]);
    let list2 = Link::create_node(vec![1]);
    let list3 = Link::create_node(vec![]);
    let node1 = swap_pairs(list1);
    let node2 = swap_pairs(list2);
    let node3 = swap_pairs(list3);
    println!("head1: {:#?}", node1);
    println!("head2: {:#?}", node2);
    println!("head3: {:#?}", node3);
    println!("----- leet code swap pairs end ------");
}

/// 测试 `K 个一组翻转链表`
fn test_reverse_k_group() {
    println!("----- leet code reverse k group start ------");
    let list1 = Link::create_node(vec![1, 2, 3, 4, 5]);
    let list2 = Link::create_node(vec![1, 2, 3, 4, 5]);
    let list3 = Link::create_node(vec![4, 8, 4]);
    let node1 = reverse_k_group(list1, 2);
    let node2 = reverse_k_group(list2, 3);
    let node3 = reverse_k_group(list3, 3);
    println!("head1: {:#?}", node1);
    println!("head2: {:#?}", node2);
    println!("head3: {:#?}", node3);
    println!("----- leet code reverse k group end ------");
}

/// 测试 `删除有序数组中的重复项`
fn test_remove_duplicates() {
    println!("----- leet code remove duplicates start ------");
    let mut nums1 = vec![1, 1, 2];
    let k1 = remove_duplicates(&mut nums1);
    println!("k: {:#?}, nums: {:?}", k1, nums1);

    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let k2 = remove_duplicates(&mut nums2);
    println!("k: {:#?}, nums: {:?}", k2, nums2);
    println!("----- leet code remove duplicates end ------");
}

pub fn test() {
    println!("----- leet code double pointer start ------");
    test_is_palindrome();
    test_max_area();
    test_remove_nth_from_end();
    test_merge_two_lists();
    test_merge_k_lists();
    test_swap_pairs();
    test_reverse_k_group();
    test_remove_duplicates();
    println!("----- leet code double pointer end ------");
}
