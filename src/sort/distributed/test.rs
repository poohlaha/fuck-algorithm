use crate::sort::distributed::bucket::bucket_sort;
use crate::sort::distributed::counting::counting_sort;

/// 测试 `计数排序`
fn test_counting() {
    println!("counting sort[n + k]:");
    let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
    let mut arr: Vec<isize> = arr.iter().map(|&x| x as isize).collect();
    counting_sort(&mut arr);
    println!("sort results: {:?}", arr);
    println!();
}

/// 测试 `桶排序`
fn test_bucket() {
    println!("bucket sort[n + k]:");
    let arr = vec![4, 10, 3, 5, 1];
    let mut arr: Vec<isize> = arr.iter().map(|&x| x as isize).collect();
    let arr = bucket_sort(&mut arr);
    println!("{:?}", arr);
    println!();
}

pub fn test() {
    println!("----- distributed sort start ------");
    test_bucket();
    test_counting();
    println!("----- distributed sort end ------");
}
