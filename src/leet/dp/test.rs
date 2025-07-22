use crate::leet::dp::regular::is_match;

/// 测试 `正则表达式匹配`
fn test_is_match() {
    println!("----- leet code matrix z start ------");
    let matched = is_match("aa".to_string(), "a".to_string());
    println!("is match: {:?}", matched);

    let matched = is_match("aa".to_string(), "a*".to_string());
    println!("is match: {:?}", matched);

    let matched = is_match("ab".to_string(), ".*".to_string());
    println!("is match: {:?}", matched);

    let matched = is_match("".to_string(), "aa".to_string());
    println!("is match: {:?}", matched);

    println!("----- leet code matrix z end ------");
}

pub fn test() {
    println!("----- leet code dp start ------");
    test_is_match();
    println!("----- leet code dp end ------");
}
