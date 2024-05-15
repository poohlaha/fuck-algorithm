use crate::math::fib::{
    coin_change, db_cycle_coin_change, db_cycle_fib, db_normal_fib, dp_coin_change, dp_fib, fib,
    length_of_lis, length_of_lis_with_two,
};

fn test_fib() {
    let str = fib(10);
    println!("fib recursion: {}", str);
}

fn test_dp_fib() {
    let str = dp_fib(10);
    println!("fib dp: {}", str);
}

fn test_db_cycle_fib() {
    let str = db_cycle_fib(10);
    println!("fib cycle dp: {}", str);
}

fn test_db_normal_fib() {
    let str = db_normal_fib(10);
    println!("fib normal dp: {}", str);
}

fn test_coin_change() {
    let str = coin_change(&vec![3, 5, 1, 4, 2, 9], 25);
    println!("coin change: {}", str);
}

fn test_dp_coin_change() {
    let str = dp_coin_change(&vec![3, 5, 1, 4, 2, 9], 25);
    println!("dp coin change: {}", str);
}

fn test_db_cycle_coin_change() {
    let str = db_cycle_coin_change(&vec![3, 5, 1, 4, 2, 9], 25);
    println!("dp cycle coin change: {}", str);
}

/// 测试 `最长递增子序列`
fn test_length_of_lis() {
    let str = length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]);
    println!("length of lis: {}", str);
}

/// 测试 `最长递增子序列, 二分查找解法`
fn test_length_of_lis_with_two() {
    let str = length_of_lis_with_two(vec![10, 9, 2, 5, 3, 7, 101, 18]);
    println!("length of lis with two: {}", str);
}

pub(crate) fn test() {
    println!("----- math start ------");
    test_fib();
    test_dp_fib();
    test_db_cycle_fib();
    test_db_normal_fib();
    test_coin_change();
    test_dp_coin_change();
    test_db_cycle_coin_change();
    test_length_of_lis();
    test_length_of_lis_with_two();
    println!("----- math end ------");
    println!();
}
