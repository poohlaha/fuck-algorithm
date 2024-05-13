use crate::math::fib::{
    coin_change, db_cycle_coin_change, db_cycle_fib, db_normal_fib, dp_coin_change, dp_fib, fib,
};

fn test_fib() {
    let str = fib(10);
    println!("fib recursion: {}", str);
}

fn test_dp_fib() {
    let str = dp_fib(10);
    println!("fib dp: {}", str);
}

fn test_db_cycle_fib() {
    let str = db_cycle_fib(10);
    println!("fib cycle dp: {}", str);
}

fn test_db_normal_fib() {
    let str = db_normal_fib(10);
    println!("fib normal dp: {}", str);
}

fn test_coin_change() {
    let str = coin_change(&vec![3, 5, 1, 4, 2, 9], 25);
    println!("coin change: {}", str);
}

fn test_dp_coin_change() {
    let str = dp_coin_change(&vec![3, 5, 1, 4, 2, 9], 25);
    println!("dp coin change: {}", str);
}

fn test_db_cycle_coin_change() {
    let str = db_cycle_coin_change(&vec![3, 5, 1, 4, 2, 9], 25);
    println!("dp cycle coin change: {}", str);
}

pub(crate) fn test() {
    println!("----- math start ------");
    test_fib();
    test_dp_fib();
    test_db_cycle_fib();
    test_db_normal_fib();
    test_coin_change();
    test_dp_coin_change();
    test_db_cycle_coin_change();
    println!("----- math end ------");
    println!();
}
