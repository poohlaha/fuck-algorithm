use crate::sort::highly::heap::{BinaryHeap, Heap};
use crate::sort::highly::merge::merge_sort;
use crate::sort::highly::quick::quick_sort;

/// 测试 `快速排序`
fn test_quick() {
    println!("quick sort[n log n]:");
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    quick_sort(&mut arr);
    println!("{:?}", arr);
    println!();
}

/// 测试 `归并排序`
fn test_merge() {
    println!("merge sort[n log n]:");
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    let arr = merge_sort(&mut arr);
    println!("{:?}", arr);
    println!();
}

/// 测试 `堆排序`
fn test_heap() {
    println!("heap sort[n log n]:");
    // let mut arr = vec![4, 10, 3, 5, 1];
    let mut heap = Heap::new();
    heap.push(4);
    heap.push(10);
    heap.push(3);
    heap.push(5);
    heap.push(1);
    heap.sort();
    println!("{:?}", heap);
    println!();
}

pub fn test() {
    println!("----- highly sort start ------");
    test_quick();
    test_merge();
    test_heap();
    println!("----- highly sort end ------");
}
