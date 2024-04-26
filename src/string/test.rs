use crate::string::str::find_min_str;

/// 测试 `最小覆盖子串`
fn test_find_min_str() {
    println!("find min str");
    let str = find_min_str("ADOBECODEBANC", "ABC");
    println!("find min str: {}", str);
}

pub(crate) fn test() {
    test_find_min_str();
}
