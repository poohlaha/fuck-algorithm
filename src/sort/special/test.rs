use crate::sort::special::cocktail::cocktail_sort;
use crate::sort::special::shell::shell_sort;

/// 测试 `希尔排序`
fn test_shell() {
    println!("shell sort[n 2]:");
    let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
    let mut arr: Vec<isize> = arr.iter().map(|&x| x as isize).collect();
    shell_sort(&mut arr);
    println!("sort results: {:?}", arr);
    println!();
}

/// 测试 `鸡尾酒排序`
fn test_cocktail() {
    println!("cocktail sort[n 2]:");
    let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
    let mut arr: Vec<isize> = arr.iter().map(|&x| x as isize).collect();
    cocktail_sort(&mut arr);
    println!("sort results: {:?}", arr);
    println!();
}

pub fn test() {
    println!("----- special sort start ------");
    test_shell();
    test_cocktail();
    println!("----- special sort end ------");
}
