use crate::math::fib::{dp_fib, fib};

fn test_fib() {
    let str = fib(10);
    println!("fib recursion: {}", str);
}

fn test_dp_fib() {
    let str = dp_fib(10);
    println!("fib dp: {}", str);
}

pub(crate) fn test() {
    println!("----- math start ------");
    test_fib();
    test_dp_fib();
    println!("----- math end ------");
    println!();
}
