use crate::string::str::{
    find_anagrams, find_min_str, is_palindrome, length_of_longest_sub_string, longest_palindrome,
    reverse_str,
};

/// 测试 `最小覆盖子串`
fn test_find_min_str() {
    let str = find_min_str("ADOBECODEBANC", "ABC");
    println!("find min str: {}", str);
}

/// 测试 `查找所有字母异位词`
fn test_find_anagrams() {
    let indexes = find_anagrams("cbaebabacd", "abc");
    println!("find anagrams: {:?}", indexes);
}

/// 测试 `最长无重复子串`
fn test_length_of_longest_sub_string() {
    let indexes = length_of_longest_sub_string("abbcaccghkdedffgggadddf");
    println!("find length of longest sub string: {:?}", indexes);
}

/// 测试 `反转字符串`
fn test_reverse_str() {
    let str = reverse_str("abcabcbbc");
    println!("reverse str: {}", str);
}

/// 测试 `回文串判断`
fn test_is_palindrome() {
    let str = is_palindrome("aba");
    println!("is palindrome: {}", str);
}

/// 测试 `查找最长回文串`
fn test_longest_palindrome() {
    let str = longest_palindrome("babadb");
    println!("longest palindrome: {}", str);
}

pub(crate) fn test() {
    println!("----- string start ------");
    test_find_min_str();
    test_find_anagrams();
    test_length_of_longest_sub_string();
    test_reverse_str();
    test_is_palindrome();
    test_longest_palindrome();
    println!("----- string end ------");
    println!();
}
