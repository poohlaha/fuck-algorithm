use crate::leet::other::divide::divide;
use crate::leet::other::remainder::smallest_repunit_div_by_k;

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

/// 测试 `可被 K 整除的最小整数`
fn test_smallest_repunit_div_by_k() {
    println!("----- leet code remainder start ------");
    let result = smallest_repunit_div_by_k(1);
    println!("{}", result);

    let result = smallest_repunit_div_by_k(2);
    println!("{}", result);

    let result = smallest_repunit_div_by_k(3);
    println!("{}", result);

    println!("----- leet code remainder end ------");
}

pub fn test() {
    println!("----- leet code other start ------");
    test_divide();
    test_smallest_repunit_div_by_k();
    println!("----- leet code other end ------");
}
