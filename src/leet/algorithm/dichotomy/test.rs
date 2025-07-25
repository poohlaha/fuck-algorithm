use crate::leet::algorithm::dichotomy::lcp::longest_common_prefix;

/// 测试 `最长公共前缀`
fn test_longest_common_prefix() {
    println!("----- leet code dichotomy lcp start ------");
    let str = longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);

    println!("lcp: {}", str);

    let str = longest_common_prefix(vec![
        "dog".to_string(),
        "racecar".to_string(),
        "car".to_string(),
    ]);

    println!("lcp: {}", str);

    let str = longest_common_prefix(vec![
        "reflower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);

    println!("lcp: {}", str);

    let str = longest_common_prefix(vec!["aa".to_string(), "ab".to_string()]);

    println!("lcp: {}", str);
    println!("----- leet code dichotomy lcp end ------");
}

pub fn test() {
    println!("----- leet code dichotomy start ------");
    test_longest_common_prefix();
    println!("----- leet code dichotomy end ------");
}
