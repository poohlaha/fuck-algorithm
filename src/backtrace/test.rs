use crate::backtrace::island::{
    closed_islands_count, count_sub_islands, islands_count, islands_count_by_bfs,
    max_area_of_island,
};
use crate::backtrace::subset::{
    can_partition_k_subsets, combination_sum, combination_sum2, combine, generate_parenthesis,
    permute_repeat, permute_unique, subsets, subsets2, subsets_with_dup,
};
use crate::backtrace::topic::solve_sudo_su;

/// 测试 `子集 - 元素无重不可复选, 以 `盒` 视角`
fn test_subsets() {
    let nums = vec![1, 2, 3];
    let results = subsets(nums);
    println!("subset box: {:?}", results);
}

/// 测试 `子集 - 元素无重不可复选, 以 `球` 视角`
fn test_subsets2() {
    let nums = vec![1, 2, 3];
    let results = subsets2(nums);
    println!("subset ball: {:?}", results);
}

/// 测试 `组合 - 元素无重不可复选`
fn test_combine() {
    let results = combine(3, 2);
    println!("combine: {:?}", results);
}

/// 测试 `子集 - 元素可重不可复选`
fn test_subsets_with_dup() {
    let nums = vec![2, 2, 1];
    let results = subsets_with_dup(nums);
    println!("subsets with dup: {:?}", results);
}

/// 测试 `从 nums 中找出中所有和为 target 的组合`
fn test_combination_sum() {
    let nums = vec![2, 5, 2, 1, 2];
    let results = combination_sum(nums, 7);
    println!("combination sum: {:?}", results);
}

/// 测试 `排列 - 元素可重不可复选`
fn test_permute_unique() {
    let nums = vec![1, 2, 2];
    let results = permute_unique(nums);
    println!("permute unique: {:?}", results);
}

/// 测试 `子集/组合 - 元素无重可复选`
fn test_combination_sum2() {
    let nums = vec![2, 5, 2, 1, 2];
    let results = combination_sum2(nums, 7);
    println!("combination sum2: {:?}", results);
}

/// 测试 `排列 - 元素无重可复选`
fn test_permute_repeat() {
    let nums = vec![1, 2, 2];
    let results = permute_repeat(nums);
    println!("permute repeat: {:?}", results);
}

/// 测试 `解独数`
fn test_solve_sudo_su() {
    /*
    let mut nums = vec![
        vec!["5", "3", "", "", "7", "", "", "", ""],
        vec!["6", "", "", "1", "9", "5", "", "", ""],
        vec!["", "9", "8", "", "", "", "", "6", ""],
        vec!["8", "", "", "", "6", "", "", "", "3"],
        vec!["4", "", "", "8", "", "3", "", "", "1"],
        vec!["7", "", "", "", "2", "", "", "", "6"],
        vec!["", "6", "", "", "", "", "2", "8", ""],
        vec!["", "", "", "4", "1", "9", "", "", "5"],
        vec!["", "", "", "", "8", "", "", "7", "9"],
    ];
     */
    let mut nums = vec![
        vec!["", "2", "", "4", "", "9", "1", "", ""],
        vec!["", "", "6", "", "5", "", "", "8", "9"],
        vec!["", "7", "", "", "8", "3", "", "2", "4"],
        vec!["7", "1", "", "5", "", "", "", "", ""],
        vec!["", "", "", "", "9", "", "2", "", ""],
        vec!["", "", "", "", "4", "", "", "", "7"],
        vec!["", "6", "", "", "", "", "", "", ""],
        vec!["", "", "7", "3", "", "", "8", "", "1"],
        vec!["3", "4", "", "", "", "5", "", "6", ""],
    ];

    solve_sudo_su(&mut nums);
    println!("solve sudo su:");
    for row in &nums {
        println!("{:?}", row);
    }
}

/// 测试 `飞地的数量`
fn test_islands_count() {
    let nums = vec![
        vec!['0', '0', '0', '0'],
        vec!['1', '0', '1', '0'],
        vec!['0', '1', '1', '0'],
        vec!['0', '0', '0', '0'],
    ];

    let count = islands_count(nums);
    println!("island count: {}", count);
}

/// 测试 `飞地的数量 - BFS`
fn test_islands_count_by_bfs() {
    let nums = vec![
        vec!['0', '0', '0', '0'],
        vec!['1', '0', '1', '0'],
        vec!['0', '1', '1', '0'],
        vec!['0', '0', '0', '0'],
    ];

    let count = islands_count_by_bfs(nums);
    println!("island count: {}", count);
}

/// 测试 `统计封闭岛屿的数目`
fn test_closed_islands_count() {
    let nums = vec![
        vec!['1', '1', '1', '1', '1', '1', '1', '0'],
        vec!['1', '0', '0', '0', '0', '1', '1', '0'],
        vec!['1', '0', '1', '0', '1', '1', '1', '0'],
        vec!['1', '0', '0', '0', '0', '1', '0', '1'],
        vec!['1', '1', '1', '1', '1', '1', '1', '0'],
    ];

    let count = closed_islands_count(nums);
    println!("closed island count: {}", count);
}

/// 测试 `岛屿的最大面积`
fn test_max_area_of_island() {
    let nums = vec![
        vec![
            '0', '0', '1', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0',
        ],
        vec![
            '0', '0', '0', '0', '0', '0', '0', '1', '1', '1', '0', '0', '0',
        ],
        vec![
            '0', '1', '1', '0', '1', '0', '0', '1', '0', '0', '0', '0', '0',
        ],
        vec![
            '0', '1', '0', '0', '1', '1', '0', '0', '1', '0', '1', '0', '0',
        ],
        vec![
            '0', '1', '0', '0', '1', '1', '0', '0', '1', '1', '1', '0', '0',
        ],
        vec![
            '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0',
        ],
        vec![
            '0', '0', '0', '0', '0', '0', '0', '1', '1', '1', '0', '0', '0',
        ],
        vec![
            '0', '0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '0',
        ],
    ];

    let count = max_area_of_island(nums);
    println!("max island area: {}", count);
}

/// 测试 `统计子岛屿`
fn test_count_sub_islands() {
    let grid1 = vec![
        vec!['1', '1', '1', '0', '0'],
        vec!['0', '1', '1', '1', '1'],
        vec!['0', '0', '0', '0', '0'],
        vec!['1', '0', '0', '0', '0'],
        vec!['1', '1', '0', '1', '1'],
    ];

    let grid2 = vec![
        vec!['1', '1', '1', '0', '0'],
        vec!['0', '0', '1', '1', '1'],
        vec!['0', '1', '0', '0', '0'],
        vec!['1', '0', '1', '1', '0'],
        vec!['0', '1', '0', '1', '0'],
    ];

    let count = count_sub_islands(grid1, grid2);
    println!("sub islands count: {}", count);

    let grid1 = vec![
        vec!['1', '0', '1', '0', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['0', '0', '0', '0', '0'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '1', '0', '1'],
    ];

    let grid2 = vec![
        vec!['0', '0', '0', '0', '0'],
        vec!['1', '1', '1', '1', '1'],
        vec!['0', '1', '0', '1', '0'],
        vec!['0', '1', '0', '1', '0'],
        vec!['1', '0', '0', '0', '1'],
    ];

    let count = count_sub_islands(grid1, grid2);
    println!("sub islands count: {}", count);
}

/// 测试 `括号生成`
fn test_generate_parenthesis() {
    let results = generate_parenthesis(3);
    println!("generate parenthesis: {:#?}", results);
}

/// 测试 `划分为k个相等的子集`
fn test_can_partition_k_subsets() {
    can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4);
}

pub(crate) fn test() {
    println!("----- backtrace start ------");
    test_subsets();
    test_subsets2();
    test_combine();
    test_subsets_with_dup();
    test_combination_sum();
    test_permute_unique();
    test_combination_sum2();
    test_permute_repeat();
    test_solve_sudo_su();
    test_islands_count();
    test_islands_count_by_bfs();
    test_closed_islands_count();
    test_max_area_of_island();
    test_count_sub_islands();
    test_generate_parenthesis();
    test_can_partition_k_subsets();
    println!("----- backtrace end ------");
    println!();
}
