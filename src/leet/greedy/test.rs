use crate::leet::greedy::{int_to_roman, roman_to_int};

/// 测试 `整数转罗马数字`
fn test_int_to_roman() {
    println!("----- leet code int to roman start ------");
    let roman = int_to_roman(3749);
    println!("3749 into roman: {:?}", roman);

    let roman = int_to_roman(58);
    println!("58 into roman: {:?}", roman);

    let roman = int_to_roman(1994);
    println!("1994 into roman: {:?}", roman);

    println!("----- leet code int to roman end ------");
}

/// 测试 `罗马数字转整数`
fn test_roman_to_int() {
    println!("----- leet code roman to int start ------");

    let num = roman_to_int("III".to_string());
    println!("III into int: {:?}", num);

    let num = roman_to_int("IV".to_string());
    println!("IV into int: {:?}", num);

    let num = roman_to_int("IX".to_string());
    println!("IX into int: {:?}", num);

    let num = roman_to_int("LVIII".to_string());
    println!("LVIII into int: {:?}", num);

    let num = roman_to_int("MCMXCIV".to_string());
    println!("MCMXCIV into int: {:?}", num);

    println!("----- leet code roman to int end ------");
}

pub fn test() {
    println!("----- leet code greedy start ------");
    test_int_to_roman();
    test_roman_to_int();
    println!("----- leet code greedy end ------");
}
