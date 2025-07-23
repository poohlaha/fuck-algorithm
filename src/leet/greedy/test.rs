use crate::leet::greedy::int_to_roman;

/// 测试 `整数转罗马数字`
fn test_int_to_roman() {
    println!("----- leet code roman start ------");
    let roman = int_to_roman(3749);
    println!("3749 into roman: {:?}", roman);

    let roman = int_to_roman(58);
    println!("58 into roman: {:?}", roman);

    let roman = int_to_roman(1994);
    println!("1994 into roman: {:?}", roman);

    println!("----- leet code roman end ------");
}

pub fn test() {
    println!("----- leet code greedy start ------");
    test_int_to_roman();
    println!("----- leet code greedy end ------");
}
