use crate::leet::backtrace::brackets::generate_parenthesis;
use crate::leet::backtrace::n::Solution;
use crate::leet::backtrace::phone::letter_combinations;
use crate::leet::backtrace::subset::count_max_or_subsets;
use crate::leet::backtrace::sudoku::Sudoku;

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

/// 测试 `括号生成`
fn test_generate_parenthesis() {
    println!("----- backtrace generate parenthesis start ------");
    let results = generate_parenthesis(3);
    println!("{:?}", results);

    let results = generate_parenthesis(1);
    println!("{:?}", results);

    println!("----- backtrace generate parenthesis end ------");
}

/// 测试 `数独`
fn test_sudoku() {
    println!("----- backtrace sudoku start ------");
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    Sudoku::solve_sudoku(&mut board);
    Sudoku::print(&board);

    println!("----- backtrace sudoku end ------");
}

/// 测试 `统计按位或能得到最大值的子集数目`
fn test_count_max_or_subsets() {
    println!("----- backtrace count max or subsets start ------");
    let results = count_max_or_subsets(vec![3, 1]);
    println!("{:?}", results);

    let results = count_max_or_subsets(vec![2, 2, 2]);
    println!("{:?}", results);

    let results = count_max_or_subsets(vec![3, 2, 1, 5]);
    println!("{:?}", results);

    println!("----- backtrace count max or subsets end ------");
}

pub fn test() {
    println!("----- leet code backtrace start ------");
    test_backtrace_n();
    test_backtrace_phone();
    test_generate_parenthesis();
    test_sudoku();
    test_count_max_or_subsets();
    println!("----- leet code backtrace end ------");
}
