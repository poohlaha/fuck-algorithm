use crate::leet::dbpointer::integer::is_palindrome;
use crate::leet::dbpointer::link::Link;
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

    let result = max_area(vec![1, 1]);
    println!("max area: {:}", result);
    println!("----- leet code max area end ------");
}

/// 测试 `删除链表的倒数第 N 个节点`
fn test_remove_nth_from_end() {
    println!("----- leet code delete link n node start ------");
    let mut head = Link::create_node(vec![1, 2, 3, 4, 5]);
    head = Link::remove_nth_from_end(head, 2);
    println!("head: {:#?}", head);
    println!("----- leet code delete link n node end ------");
}

pub fn test() {
    println!("----- leet code double pointer start ------");
    test_is_palindrome();
    test_max_area();
    test_remove_nth_from_end();
    println!("----- leet code double pointer end ------");
}
