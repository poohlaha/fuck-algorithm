use crate::leet::algorithm::dichotomy::lcp::longest_common_prefix;
use crate::leet::algorithm::dichotomy::ship::Ship;

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

fn test_ship() {
    println!("----- leet code ship start ------");
    let weights: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = Ship::ship_within_days(weights, 5);
    println!("result: {}", result);

    let weights: Vec<i32> = vec![3, 2, 2, 4, 1, 4];
    let result = Ship::ship_within_days(weights, 3);
    println!("result: {}", result);

    let weights: Vec<i32> = vec![1, 2, 3, 1, 1];
    let result = Ship::ship_within_days(weights, 4);
    println!("result: {}", result);

    println!("----- leet code ship start ------");
}

pub fn test() {
    println!("----- leet code dichotomy start ------");
    test_longest_common_prefix();
    test_ship();
    println!("----- leet code dichotomy end ------");
}
