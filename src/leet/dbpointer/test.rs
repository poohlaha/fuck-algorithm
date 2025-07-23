use crate::leet::dbpointer::integer::is_palindrome;
use crate::leet::dbpointer::max_area;

/// 测试 `回文数`
fn test_is_palindrome() {
    println!("----- leet code palindrome start ------");
    let result = is_palindrome(121);
    println!("is palindrome: {:}", result);

    let result = is_palindrome(-121);
    println!("is palindrome: {:}", result);

    let result = is_palindrome(3);
    println!("is palindrome: {:}", result);

    let result = is_palindrome(10);
    println!("is palindrome: {:}", result);

    let result = is_palindrome(21120);
    println!("is palindrome: {:}", result);

    println!("----- leet code palindrome end ------");
}

/// 测试 `盛最多水的容器`
fn test_max_area() {
    println!("----- leet code max area start ------");
    let result = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("max area: {:}", result);
    println!("----- leet code max area end ------");
}

pub fn test() {
    println!("----- leet code double pointer start ------");
    test_is_palindrome();
    test_max_area();
    println!("----- leet code double pointer end ------");
}
