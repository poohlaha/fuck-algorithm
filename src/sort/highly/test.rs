use crate::sort::highly::quick::quick_sort;

/// 测试 `快速排序`
fn test_quick() {
    println!("quick sort[n log n]:");
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    quick_sort(&mut arr);
    println!("{:?}", arr);
    println!();
}

pub fn test() {
    println!("----- highly sort start ------");
    test_quick();
    println!("----- highly sort end ------");
}
