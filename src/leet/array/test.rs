use crate::leet::array::median::find_median_sorted_arrays;
use crate::leet::array::region::NumArray;
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

/// 测试 `区域和检索-数组不可变`
fn test_region() {
    println!("----- leet code array region start ------");
    let nums = vec![-2, 0, 3, -5, 2, -1];
    let region = NumArray::new(nums);
    let sum = region.sum_range(0, 2);
    println!("0 到 2 的区间和: {}", sum);

    let sum = region.sum_range(2, 5);
    println!("2 到 5 的区间和: {}", sum);

    let sum = region.sum_range(5, 2);
    println!("5 到 2 的区间和: {}", sum);

    let sum = region.sum_range(0, 5);
    println!("0 到 5 的区间和: {}", sum);

    let sum = region.sum_range(3, 3);
    println!("3 到 3 的区间和: {}", sum);
    println!("----- leet code array region end ------");
}

/// 测试 `区间加法`
fn test_difference() {
    println!("----- leet code array difference start ------");
    let nums = vec![0, 0, 0, 0, 0];
    let mut diff = crate::leet::array::difference::NumArray::new(nums);
    diff.increment(1, 3, 2);
    let arr = diff.restore();
    println!("对区间 [1, 3] 增加 2 后: {:?}", arr);

    diff.increment(2, 4, 3);
    let arr = diff.restore();
    println!("对区间 [2, 4] 增加 3 后: {:?}", arr);

    diff.increment(0, 2, -2);
    let arr = diff.restore();
    println!("对区间 [0, 2] 增加 (-1) 后: {:?}", arr);

    let flag = crate::leet::array::difference::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4);
    println!("[[2, 1, 5], [3, 3, 7]], capacity = 4: {}", flag);

    let flag = crate::leet::array::difference::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5);
    println!("[[2, 1, 5], [3, 3, 7]], capacity = 5: {}", flag);

    let flag = crate::leet::array::difference::car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3);
    println!("[[2, 1, 5], [3, 5, 7]], capacity = 3: {}", flag);

    let arr = crate::leet::array::difference::corp_flight_bookings(
        vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]],
        5,
    );

    println!(
        "航班: [[1, 2, 10], [2, 3, 20], [2, 5, 25], n = 5: {:?}",
        arr
    );

    let arr = crate::leet::array::difference::corp_flight_bookings(
        vec![vec![1, 2, 10], vec![2, 2, 15]],
        2,
    );

    println!("航班: [[1, 2, 10], [2, 2, 15], n = 2: {:?}", arr);

    println!("----- leet code array difference end ------");
}

/// 测试 `N 数之和`
fn test_n_sum() {
    println!("----- leet code array n sum start ------");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let results = Array::n_sum(&nums, target, 2, 0);
    println!("results: {:?}", results);

    let nums = vec![3, 3];
    let target = 6;
    let results = Array::n_sum(&nums, target, 2, 0);
    println!("results: {:?}", results);

    let nums = vec![-4, -1, -1, 0, 1, 2];
    let results = Array::n_sum(&nums, 0, 3, 0);
    println!("results: {:?}", results);

    println!("----- leet code array n sum end ------");
}

/// 测试 `寻找两个正序数组的中位数`
fn test_find_median_sorted_arrays() {
    println!("----- leet code array media start ------");
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let media = find_median_sorted_arrays(nums1, nums2);
    println!("中位数: {}", media);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    let media = find_median_sorted_arrays(nums1, nums2);
    println!("中位数: {}", media);

    let nums1 = vec![1];
    let nums2 = vec![2];
    let media = find_median_sorted_arrays(nums1, nums2);
    println!("中位数: {}", media);

    let nums1 = vec![];
    let nums2 = vec![1];
    let media = find_median_sorted_arrays(nums1, nums2);
    println!("中位数: {}", media);

    let nums1 = vec![];
    let nums2 = vec![2, 3];
    let media = find_median_sorted_arrays(nums1, nums2);
    println!("中位数: {}", media);

    println!("----- leet code array media end ------");
}

pub fn test() {
    println!("----- leet code array start ------");
    test_two_sum_2();
    test_two_sum();
    test_three_sum();
    test_four_sum();
    test_region();
    test_difference();
    test_n_sum();
    test_find_median_sorted_arrays();
    println!("----- leet code array end ------");
}
