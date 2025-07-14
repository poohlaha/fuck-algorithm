use crate::leet::str::integer::{my_atoi, reverse};
use crate::leet::str::z;

/// 测试 `Z 字形变换`
fn test_string_z_convert() {
    println!("----- leet code string z start ------");
    let result = z::convert("PAYPALISHIRING".to_string(), 3);
    println!("z convert: {:}", result);

    let result = z::convert("PAYPALISHIRING".to_string(), 4);
    println!("z convert: {:}", result);

    println!("----- leet code string z end ------");
}

/// 测试 `整数反转`
fn test_integer_reverse() {
    println!("----- leet code string integer reverse start ------");
    let result = reverse(123);
    println!("integer reverse: {:}", result);

    let result = reverse(-123);
    println!("integer reverse: {:}", result);

    let result = reverse(0);
    println!("integer reverse: {:}", result);

    let result = reverse(2147483647);
    println!("integer reverse: {:}", result);
    println!("----- leet code string integer reverse end ------");
}

/// 测试 `字符串转换整数`
fn test_my_atoi() {
    println!("----- leet code string my atoi start ------");
    let result = my_atoi("42".to_string());
    println!("my atoi reverse: {:}", result);

    let result = my_atoi(" -042".to_string());
    println!("my atoi reverse: {:}", result);

    let result = my_atoi("1337c0d3".to_string());
    println!("my atoi reverse: {:}", result);

    let result = my_atoi("0-1".to_string());
    println!("my atoi reverse: {:}", result);
    println!("----- leet code string my atoi end ------");
}

pub fn test() {
    println!("----- leet code string start ------");
    test_string_z_convert();
    test_integer_reverse();
    test_my_atoi();
    println!("----- leet code string end ------");
}
