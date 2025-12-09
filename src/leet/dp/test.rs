use crate::leet::dp::max::max_value;
use crate::leet::dp::regular::is_match;

/// 测试 `正则表达式匹配`
fn test_is_match() {
    println!("----- leet code reg match start ------");
    let matched = is_match("aa".to_string(), "a".to_string());
    println!("is match: {:?}", matched);

    let matched = is_match("aa".to_string(), "a*".to_string());
    println!("is match: {:?}", matched);

    let matched = is_match("ab".to_string(), ".*".to_string());
    println!("is match: {:?}", matched);

    let matched = is_match("".to_string(), "aa".to_string());
    println!("is match: {:?}", matched);

    println!("----- leet code reg match end ------");
}

/// 测试 `最多可以参加的会议数目 II`
fn test_max_value() {
    println!("----- leet code reg match start ------");
    let result = max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2);
    println!("result: {:?}", result);

    let result = max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]], 2);
    println!("result: {:?}", result);

    let result = max_value(
        vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]],
        3,
    );
    println!("result: {:?}", result);

    println!("----- leet code reg match end ------");
}

pub fn test() {
    println!("----- leet code dp start ------");
    test_is_match();
    test_max_value();
    println!("----- leet code dp end ------");
}
