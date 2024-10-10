use crate::backtrace::subset::{
    combination_sum, combination_sum2, combine, permute_repeat, permute_unique, subsets, subsets2,
    subsets_with_dup,
};
use crate::backtrace::topic::solve_sudo_su;

/// 测试 `子集 - 元素无重不可复选, 以 `盒` 视角`
fn test_subsets() {
    let nums = vec![1, 2, 3];
    let results = subsets(nums);
    println!("subset box: {:?}", results);
}

/// 测试 `子集 - 元素无重不可复选, 以 `球` 视角`
fn test_subsets2() {
    let nums = vec![1, 2, 3];
    let results = subsets2(nums);
    println!("subset ball: {:?}", results);
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

/// 测试 `解独数`
fn test_solve_sudo_su() {
    /*
    let mut nums = vec![
        vec!["5", "3", "", "", "7", "", "", "", ""],
        vec!["6", "", "", "1", "9", "5", "", "", ""],
        vec!["", "9", "8", "", "", "", "", "6", ""],
        vec!["8", "", "", "", "6", "", "", "", "3"],
        vec!["4", "", "", "8", "", "3", "", "", "1"],
        vec!["7", "", "", "", "2", "", "", "", "6"],
        vec!["", "6", "", "", "", "", "2", "8", ""],
        vec!["", "", "", "4", "1", "9", "", "", "5"],
        vec!["", "", "", "", "8", "", "", "7", "9"],
    ];
     */
    let mut nums = vec![
        vec!["", "2", "", "4", "", "9", "1", "", ""],
        vec!["", "", "6", "", "5", "", "", "8", "9"],
        vec!["", "7", "", "", "8", "3", "", "2", "4"],
        vec!["7", "1", "", "5", "", "", "", "", ""],
        vec!["", "", "", "", "9", "", "2", "", ""],
        vec!["", "", "", "", "4", "", "", "", "7"],
        vec!["", "6", "", "", "", "", "", "", ""],
        vec!["", "", "7", "3", "", "", "8", "", "1"],
        vec!["3", "4", "", "", "", "5", "", "6", ""],
    ];

    solve_sudo_su(&mut nums);
    println!("solve sudo su:");
    for row in &nums {
        println!("{:?}", row);
    }
}

pub(crate) fn test() {
    println!("----- backtrace start ------");
    test_subsets();
    test_subsets2();
    test_combine();
    test_subsets_with_dup();
    test_combination_sum();
    test_permute_unique();
    test_combination_sum2();
    test_permute_repeat();
    test_solve_sudo_su();
    println!("----- backtrace end ------");
    println!();
}
