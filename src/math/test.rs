use crate::math::fib::{db_cycle_fib, db_normal_fib, dp_fib, fib};

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

pub(crate) fn test() {
    println!("----- math start ------");
    test_fib();
    test_dp_fib();
    test_db_cycle_fib();
    test_db_normal_fib();
    println!("----- math end ------");
    println!();
}
