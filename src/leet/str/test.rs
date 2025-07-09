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

pub fn test() {
    println!("----- leet code string start ------");
    test_string_z_convert();
    println!("----- leet code string end ------");
}
