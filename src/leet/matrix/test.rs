use crate::leet::matrix::difference::Difference;
use crate::leet::matrix::region::Region;

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

pub fn test() {
    println!("----- leet code link start ------");
    test_region_query();
    test_difference();
    println!("----- leet code link end ------");
}
