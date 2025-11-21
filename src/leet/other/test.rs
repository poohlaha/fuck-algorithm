use crate::leet::other::divide::divide;

/// 测试 `两数相除`
fn test_divide() {
    println!("----- leet code divide start ------");
    let result = divide(10, 3);
    println!("{}", result);

    let result = divide(7, -3);
    println!("{}", result);

    let result = divide(-2147483648, 1);
    println!("{}", result);
    println!("----- leet code divide end ------");
}

pub fn test() {
    println!("----- leet code other start ------");
    test_divide();
    println!("----- leet code other end ------");
}
