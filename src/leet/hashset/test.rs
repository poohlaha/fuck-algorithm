use crate::leet::hashset::find_final_value;

/// 测试 `将找到的值乘以 2`
fn test_final_value() {
    println!("----- leet code final value start ------");
    let result = find_final_value(vec![5, 3, 6, 1, 12], 3);
    println!("{:}", result);

    let result = find_final_value(vec![2, 7, 9], 4);
    println!("{:}", result);
    println!("----- leet code final value end ------");
}

pub fn test() {
    println!("----- leet code other start ------");
    test_final_value();
    println!("----- leet code other end ------");
}
