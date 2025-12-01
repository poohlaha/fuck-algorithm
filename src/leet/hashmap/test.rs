use crate::leet::hashmap::find_lucky;

/// 测试 `找出数组中的幸运数`
fn test_final_lucky() {
    println!("----- leet code final lucky start ------");
    let result = find_lucky(vec![2, 2, 3, 4]);
    println!("{:}", result);

    let result = find_lucky(vec![1, 2, 2, 3, 3, 3]);
    println!("{:}", result);

    let result = find_lucky(vec![2, 2, 2, 3, 3]);
    println!("{:}", result);

    let result = find_lucky(vec![5]);
    println!("{:}", result);

    let result = find_lucky(vec![7, 7, 7, 7, 7, 7, 7]);
    println!("{:}", result);

    println!("----- leet code final lucky end ------");
}

pub fn test() {
    println!("----- leet code hashmap start ------");
    test_final_lucky();
    println!("----- leet code hashmap end ------");
}
