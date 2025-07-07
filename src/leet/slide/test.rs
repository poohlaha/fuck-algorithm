use crate::leet::slide::length_of_longest_substring;

/// 测试 ``
fn test_slide_longest_substring() {
    println!("----- leet code slide length of longest substring start ------");
    let result = length_of_longest_substring("abcabcbb".to_string());
    println!("result: {}", result);

    let result = length_of_longest_substring("bbbbb".to_string());
    println!("result: {}", result);

    let result = length_of_longest_substring("pwwkew".to_string());
    println!("result: {}", result);
    println!("----- leet code slide length of longest substring end ------");
}

pub fn test() {
    println!("----- leet code array start ------");
    test_slide_longest_substring();
    println!("----- leet code array end ------");
}
