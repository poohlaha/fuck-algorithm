use crate::string::str::{find_anagrams, find_min_str, length_of_longest_sub_string};

/// 测试 `最小覆盖子串`
fn test_find_min_str() {
    println!("find min str");
    let str = find_min_str("ADOBECODEBANC", "ABC");
    println!("find min str: {}", str);
}

/// 测试 `查找所有字母异位词`
fn test_find_anagrams() {
    println!("find anagrams");
    let indexes = find_anagrams("cbaebabacd", "abc");
    println!("find anagrams: {:?}", indexes);
}

/// 测试 `最长无重复子串`
fn test_length_of_longest_sub_string() {
    println!("find length of longest sub string");
    let indexes = length_of_longest_sub_string("abcabcbb");
    println!("find length of longest sub string: {:?}", indexes);
}

pub(crate) fn test() {
    test_find_min_str();
    test_find_anagrams();
    test_length_of_longest_sub_string()
}
