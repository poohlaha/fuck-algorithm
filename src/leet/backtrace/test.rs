use crate::leet::backtrace::n::Solution;
use crate::leet::backtrace::phone::letter_combinations;

/// 测试 `N 皇后问题`
fn test_backtrace_n() {
    println!("----- backtrace n start ------");
    let results = Solution::solve_n_queens(4);
    println!("{:#?}", results);

    let results = Solution::solve_n_queens(2);
    println!("{:#?}", results);

    let results = Solution::solve_n_queens(3);
    println!("{:#?}", results);
    println!("----- backtrace n end ------");
}

/// 测试 `电话号码的字母组合`
fn test_backtrace_phone() {
    println!("----- backtrace phone start ------");
    let results = letter_combinations("23".to_string());
    println!("{:?}", results);

    let results = letter_combinations(String::new());
    println!("{:?}", results);

    let results = letter_combinations("2".to_string());
    println!("{:?}", results);

    println!("----- backtrace phone end ------");
}

pub fn test() {
    println!("----- leet code backtrace start ------");
    test_backtrace_n();
    test_backtrace_phone();
    println!("----- leet code backtrace end ------");
}
