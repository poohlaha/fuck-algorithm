use crate::leet::matrix::difference::Difference;
use crate::leet::matrix::divisible::{number_of_paths, number_of_paths_scroll};
use crate::leet::matrix::region::Region;
use crate::leet::matrix::z::find_diagonal_order;

/// 测试 `二维前缀和`
fn test_region_query() {
    println!("----- leet code matrix region start ------");
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];

    let region = Region::new(matrix);
    let sum = region.sum_range(2, 1, 4, 3);
    println!("(2, 1) 到 (4, 3) 的区间和: {}", sum);

    let sum = region.sum_range(1, 1, 2, 2);
    println!("(1, 1) 到 (2, 2) 的区间和: {}", sum);

    let sum = region.sum_range(1, 2, 2, 4);
    println!("(1, 2) 到 (2, 4) 的区间和: {}", sum);

    let sum = region.sum_range(2, 4, 1, 2);
    println!("(2, 4) 到 (1, 2) 的区间和: {}", sum);

    let sum = region.sum_range(3, 3, 3, 3);
    println!("(3, 3) 到 (3, 3) 的区间和: {}", sum);

    let matrix = vec![
        vec![
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
        ],
        vec![
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
        ],
        vec![
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
        ],
        vec![
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
        ],
        vec![
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
        ],
    ];

    let region = Region::new(matrix);
    let sum = region.sum_range(1, 1, 4, 4);
    println!("极限测试: (1, 1) 到 (4, 4) 的区间和: {}", sum);
    println!("----- leet code matrix region end ------");
}

/// 测试 `二维差分数组`
fn test_difference() {
    println!("----- leet code matrix difference start ------");
    let arr = Difference::range_add_queries(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]]);
    println!("[1, 1, 2, 2], [0, 0, 1, 1] n = 3: {:?}", arr);

    let arr = Difference::range_add_queries(2, vec![vec![0, 0, 1, 1]]);
    println!("[0, 0, 1, 1] n = 2: {:?}", arr);

    let arr = Difference::range_add_queries(2, vec![vec![2, 2, 2, 2]]);
    println!("[2, 2, 2, 2] n = 2: {:?}", arr);
    println!("----- leet code matrix difference end ------");
}

/// 测试 `矩阵 Z 形变换`
fn test_find_diagonal_order() {
    println!("----- leet code matrix z start ------");
    let arr = find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    println!("arr: {:?}", arr);

    let arr = find_diagonal_order(vec![vec![1, 2], vec![3, 4]]);
    println!("arr: {:?}", arr);

    let arr = find_diagonal_order(vec![vec![1, 2]]);
    println!("arr: {:?}", arr);

    println!("----- leet code matrix z end ------");
}

/// 测试 `矩阵中和能被 K 整除的路径`
fn test_number_of_paths() {
    println!("----- leet code matrix divisible start ------");
    let result = number_of_paths(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3);
    println!("result: {:}", result);

    let result = number_of_paths(vec![vec![0, 0]], 5);
    println!("result: {:}", result);

    let result = number_of_paths(
        vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]],
        1,
    );
    println!("result: {:}", result);

    let result = number_of_paths_scroll(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3);
    println!("scroll result: {:}", result);

    let result = number_of_paths_scroll(vec![vec![0, 0]], 5);
    println!("scroll result: {:}", result);

    let result = number_of_paths_scroll(
        vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]],
        1,
    );
    println!("scroll result: {:}", result);

    println!("----- leet code matrix divisible end ------");
}

pub fn test() {
    println!("----- leet code link start ------");
    test_region_query();
    test_difference();
    test_find_diagonal_order();
    test_number_of_paths();
    println!("----- leet code link end ------");
}
