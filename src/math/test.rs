use crate::math::fib::{coin_change, db_cycle_coin_change, db_cycle_fib, db_normal_fib, dp_coin_change, dp_fib, fib, length_of_lis, length_of_lis_with_two, longest_common_subsequence, max_envelopes, max_sub_array, max_sub_array_dynamic, max_sub_array_kadane};

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
    let (str, lis) = length_of_lis_with_two(vec![10, 9, 2, 5, 3, 7, 101, 18]);
    println!("length of lis with two: {}, lis: {:?}", str, lis);
}

/// 测试 `俄罗斯套娃信封问题`
fn test_max_envelopes() {
    let str = max_envelopes(vec![(5, 4), (6, 4), (6, 7), (2, 3)]);
    println!("max envelopes: {}", str);
}

/// 测试 `最大子数组和(滑动窗口算法)`
fn test_max_sub_array() {
    let str = max_sub_array(vec![-3, 1, 3, -1, 2, -4, 2]);
    println!("max sub array: {}", str);
}

/// 测试 `最大子数组和(Kadane算法, 动态规划算法的一种)`
fn test_max_sub_array_kadane() {
    let str = max_sub_array_kadane(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
    println!("max sub array kadane: {}", str);
}

/// 测试 `最大子数组和(动态规划算法)`
fn test_max_sub_array_dynamic() {
    let str = max_sub_array_dynamic(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
    println!("max sub array dynamic: {}", str);
}

/// 测试 `最长公共子序列`
fn test_longest_common_subsequence() {
    let str = longest_common_subsequence("ABCBDAB", "BDCAB");
    println!("longest common subsequence: {}", str);
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
    test_max_envelopes();
    test_max_sub_array();
    test_max_sub_array_kadane();
    test_max_sub_array_dynamic();
    test_longest_common_subsequence();
    println!("----- math end ------");
    println!();
}
