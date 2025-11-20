use crate::leet::algorithm::kmp::subscript::Subscript;

/// 测试 `找出字符串中第一个匹配项的下标`
fn test_str_str() {
    println!("----- leet code find subscript start ------");
    /*
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();
    let n = Subscript::str_str(haystack, needle);
    println!("{}", n);

    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();
    let n = Subscript::str_str(haystack, needle);
    println!("{}", n);
     */

    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();
    let n = Subscript::str_str_qs(haystack, needle);
    println!("{}", n);

    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();
    let n = Subscript::str_str_qs(haystack, needle);
    println!("{}", n);

    println!("----- leet code find subscriptt end ------");
}

pub fn test() {
    println!("----- leet code kmp start ------");
    test_str_str();
    println!("----- leet code kmp end ------");
}
