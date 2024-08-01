use crate::backtrace::subset::{combine, subsets};

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

pub(crate) fn test() {
    println!("----- backtrace start ------");
    test_subsets();
    test_combine();
    println!("----- backtrace end ------");
    println!();
}
