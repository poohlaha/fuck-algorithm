use crate::sort::basic::bubble::bubble_sort;
use crate::sort::basic::insertion::insertion_sort;
use crate::sort::basic::selection::selection_sort;

/// 测试 `冒泡排序`
pub fn test_bubble() {
    println!("bubble sort[O(n2)]:");
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut arr);
    println!("{:?}", arr);
    println!();
}

/// 测试 `插入排序`
pub fn test_insertion() {
    println!("insertion sort[O(n2)]:");
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
    println!();
}

/// 测试 `选择排序`
pub fn test_selection() {
    println!("selection sort[O(n2)]:");
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    selection_sort(&mut arr);
    println!("{:?}", arr);
    println!();
}

pub fn test() {
    println!("----- basic sort start ------");
    test_bubble();
    test_insertion();
    test_selection();
    println!("----- basic sort end ------");
}
