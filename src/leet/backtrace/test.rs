/// 测试 `N 皇后问题`
fn test_backtrace_n() {
    println!("----- backtrace n start ------");
    let results = crate::leet::backtrace::n::Solution::solve_n_queens(4);
    println!("{:#?}", results);

    let results = crate::leet::backtrace::n::Solution::solve_n_queens(2);
    println!("{:#?}", results);

    let results = crate::leet::backtrace::n::Solution::solve_n_queens(3);
    println!("{:#?}", results);
    println!("----- backtrace n end ------");
}

pub fn test() {
    println!("----- leet code backtrace start ------");
    test_backtrace_n();
    println!("----- leet code backtrace end ------");
}
