use crate::bfs::{bidirectional_open_lock, open_lock, sliding_puzzle};

/// 测试 `解开密码锁的最少次数(打开转盘锁)`
pub(crate) fn test_open_lock() {
    let deadends = vec!["0201", "0101", "0102", "1212", "2002"];
    let target = "0202";
    let dept = open_lock(deadends.clone(), target);
    let dept2 = bidirectional_open_lock(deadends.clone(), target);
    println!("open lock: {}\n", dept);
    println!("bidirectional open lock: {}\n", dept2);
}

/// 测试 `滑动谜题`
pub(crate) fn test_sliding_puzzle() {
    let nums: Vec<Vec<usize>> = vec![vec![4, 1, 2], vec![5, 0, 3]];
    let target: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![4, 5, 0]];

    let step = sliding_puzzle(nums, target, 2, 3);
    println!("sliding puzzle step: {}", step);
}

pub(crate) fn test() {
    println!("----- BFS start ------");
    test_open_lock();
    test_sliding_puzzle();
    println!("----- BFS end ------");
    println!();
}
