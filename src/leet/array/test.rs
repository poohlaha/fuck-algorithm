use crate::leet::array::Array;

/// 测试 `两数之和II - 输入有序数组`
fn test_two_sum_2() {
    println!("----- leet code array two sum 2 start ------");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let interval = Array::two_sum_2(nums, target);
    println!("interval: {:?}", interval);

    let nums = vec![0, 0, 3, 4];
    let target = 0;
    let interval = Array::two_sum_2(nums, target);
    println!("interval: {:?}", interval);
    println!("----- leet code array two sum 2 end ------");
}

/// 测试 `两数之和`
fn test_two_sum() {
    println!("----- leet code array two sum start ------");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let interval = Array::two_sum_hash(nums, target);
    println!("interval: {:?}", interval);

    let nums = vec![0, 0, 3, 4];
    let target = 0;
    let interval = Array::two_sum(nums, target);
    println!("interval: {:?}", interval);
    println!("----- leet code array two sum end ------");
}

/// 测试 `三数之和`
fn test_three_sum() {
    println!("----- leet code array three sum start ------");
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let results = Array::three_sum(nums);
    println!("results: {:?}", results);

    let nums = vec![-2, 0, 1, 1, 2];
    let results = Array::three_sum(nums);
    println!("results: {:?}", results);
    println!("----- leet code array three sum end ------");
}

/// 测试 `四数之和`
fn test_four_sum() {
    println!("----- leet code array four sum start ------");
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let results = Array::four_sum(nums, target);
    println!("results: {:?}", results);

    let nums = vec![2, 2, 2, 2, 2];
    let target = 8;
    let results = Array::four_sum(nums, target);
    println!("results: {:?}", results);

    let nums = vec![0, 0, 0, 1000000000, 1000000000, 1000000000, 1000000000];
    let target = 1000000000;
    let results = Array::four_sum(nums, target);
    println!("results: {:?}", results);
    println!("----- leet code array four sum end ------");
}

pub fn test() {
    println!("----- leet code array start ------");
    test_two_sum_2();
    test_two_sum();
    test_three_sum();
    test_four_sum();
    println!("----- leet code array end ------");
}
