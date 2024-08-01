use crate::backtrace::subset::{
    combination_sum, combination_sum2, combine, permute_repeat, permute_unique, subsets,
    subsets_with_dup,
};

/// 测试 `子集 - 元素无重不可复选`
fn test_subsets() {
    let nums = vec![1, 2, 3];
    let results = subsets(nums);
    println!("subset: {:?}", results);
}

/// 测试 `组合 - 元素无重不可复选`
fn test_combine() {
    let results = combine(3, 2);
    println!("combine: {:?}", results);
}

/// 测试 `子集 - 元素可重不可复选`
fn test_subsets_with_dup() {
    let nums = vec![2, 2, 1];
    let results = subsets_with_dup(nums);
    println!("subsets with dup: {:?}", results);
}

/// 测试 `从 nums 中找出中所有和为 target 的组合`
fn test_combination_sum() {
    let nums = vec![2, 5, 2, 1, 2];
    let results = combination_sum(nums, 7);
    println!("combination sum: {:?}", results);
}

/// 测试 `排列 - 元素可重不可复选`
fn test_permute_unique() {
    let nums = vec![1, 2, 2];
    let results = permute_unique(nums);
    println!("permute unique: {:?}", results);
}

/// 测试 `子集/组合 - 元素无重可复选`
fn test_combination_sum2() {
    let nums = vec![2, 5, 2, 1, 2];
    let results = combination_sum2(nums, 7);
    println!("combination sum2: {:?}", results);
}

/// 测试 `排列 - 元素无重可复选`
fn test_permute_repeat() {
    let nums = vec![1, 2, 2];
    let results = permute_repeat(nums);
    println!("permute repeat: {:?}", results);
}

pub(crate) fn test() {
    println!("----- backtrace start ------");
    test_subsets();
    test_combine();
    test_subsets_with_dup();
    test_combination_sum();
    test_permute_unique();
    test_combination_sum2();
    test_permute_repeat();
    println!("----- backtrace end ------");
    println!();
}
