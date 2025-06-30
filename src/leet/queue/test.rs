use crate::leet::queue::monotonic::Monotonic;

/// 测试 `单调队列`
fn test_queue_monotonic() {
    println!("----- monotonic start ------");
    let arr = Monotonic::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
    println!("窗口内最大值: {:?}", arr);

    let arr = Monotonic::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 1);
    println!("窗口内最大值: {:?}", arr);

    let arr = Monotonic::max_sliding_window2(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
    println!("窗口内最大值: {:?}", arr);

    let arr = Monotonic::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0);
    println!("绝对差不超过限制 {}: {:?}", 4, arr);
    println!("----- monotonic end ------");
}

pub fn test() {
    println!("----- leet code queue start ------");
    test_queue_monotonic();
    println!("----- leet code queue end ------");
}
