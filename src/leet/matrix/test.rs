use crate::leet::matrix::region::Region;

/// 测试 ``
fn test_region_query() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];

    let region = Region::new(matrix);
    let sum = region.query(2, 1, 4, 3);
    println!("(2, 1) 到 (4, 3) 的区间值: {}", sum);

    let sum = region.query(1, 1, 2, 2);
    println!("(1, 1) 到 (2, 2) 的区间值: {}", sum);

    let sum = region.query(1, 2, 2, 4);
    println!("(1, 2) 到 (2, 4) 的区间值: {}", sum);

    let sum = region.query(2, 4, 1, 2);
    println!("(2, 4) 到 (1, 2) 的区间值: {}", sum);

    let sum = region.query(3, 3, 3, 3);
    println!("(3, 3) 到 (3, 3) 的区间值: {}", sum);

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
    let sum = region.query(1, 1, 4, 4);
    println!("极限测试: (1, 1) 到 (4, 4) 的区间值: {}", sum);
}

pub fn test() {
    println!("----- leet code link start ------");
    test_region_query();
    println!("----- leet code link end ------");
}
