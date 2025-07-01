use crate::leet::stack::Stack;

/// 测试 `有效的括号`
fn test_stack_validate_bracket() {
    println!("----- validate bracket start ------");
    let flag = Stack::is_valid("()".to_string());
    println!("是否是有效的括号: {}", flag);

    let flag = Stack::is_valid("()[]{}".to_string());
    println!("是否是有效的括号: {}", flag);

    let flag = Stack::is_valid("(]".to_string());
    println!("是否是有效的括号: {}", flag);

    let flag = Stack::is_valid("([])".to_string());
    println!("是否是有效的括号: {}", flag);

    let flag = Stack::is_valid(")(){}".to_string());
    println!("是否是有效的括号: {}", flag);

    let flag = Stack::is_valid("(])".to_string());
    println!("是否是有效的括号: {}", flag);

    println!("----- validate bracket end ------");
}

pub fn test() {
    println!("----- leet code stack start ------");
    test_stack_validate_bracket();
    println!("----- leet code stack end ------");
}
