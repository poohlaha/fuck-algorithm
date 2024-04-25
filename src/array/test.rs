use crate::array::arr::{binary_search, move_k_element, remove_duplicates, remove_k_element};

/// 测试 `删除有序数组中的重复项`
fn test_remove_duplicates() {
    println!("remove duplicates");
    let (len, arr) = remove_duplicates(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 5, 5]);
    println!("remove duplicates new len: {}", len);
    println!("remove duplicates new array: {:?}", arr);
}

/// 测试 `移除所有数值等于 k 的元素`
fn test_remove_k_element() {
    println!("remove k element");
    let (len, arr) = remove_k_element(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 5, 5], 1);
    println!("remove k element new len: {}", len);
    println!("remove k element new array: {:?}", arr);
}

/// 测试 `将数组中的所有为 k 的元素移动到数组末尾`
fn test_move_k_element() {
    println!("move k element");
    let arr = move_k_element(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 5, 5], 1);
    println!("move k element new array: {:?}", arr);
}

/// 测试 `二分查找`
fn test_binary_search() {
    println!("binary search");
    let index = binary_search(vec![1, 2, 3, 4, 5, 6, 7, 8], 3);
    println!("binary search find index: {}", index);
}

pub(crate) fn test() {
    test_remove_duplicates();
    test_remove_k_element();
    test_move_k_element();
    test_binary_search();
}
