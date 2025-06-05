/**
  二分查找算法
  - 可行 ➜ 往右逼近; 不可行 ➜ 往左逼近

  举例：
   木村切割问题:
   - 有多根木头 。我们想切成 `k` 段，要求每段长度不小于 `x`。问 `x` 最大是多少？
   - 我们无法直接找到“x = ?”，但我们能判断: `如果每段长度 ≥ mid，能否切成 k 段？`, 用二分查找 `x`

   假设：
    - 木头长度: [7, 9, 13, 17]
    - 需要切成 k = 5 段
    问题: 求最大化每段的最小长度
*/

fn max_min_length(woods: Vec<i32>, k: i32) -> i32 {
    // 1. 二分的区间
    let mut low = 1;
    let mut high = *woods.iter().max().unwrap();
    let mut result = 0;

    while low <= high {
        // 2. 先确定 `中间位置`，把数组（或区间）分成 `左` `右` 两部分
        // let mid = (low + high) / 2; 可能导致溢出，结果可能变负数，二分死循环、程序崩溃
        // 当 low 和 high 特别大（接近 i32::MAX 或 i64::MAX）
        let mid = low + (high - low) / 2;

        // 3. 木头能不能切成 `k` 段, 且每段长度 >= mid
        let mut count = 0;
        for &wood in &woods {
            count += wood / mid
        }

        if count >= k {
            // 可行 ➜ 往右逼近
            result = mid;
            low = mid + 1;
        } else {
            // 不可行 ➜ 往左逼近
            high = mid - 1;
        }
    }

    result
}
