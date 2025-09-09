/*!
  双指针
*/

use std::cmp::{max, min};

mod arr;
mod integer;
mod link;
pub mod test;

/**
   11. 盛最多水的容器
   力扣: https://leetcode.cn/problems/container-with-most-water/description/
   题目: 给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。
        找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
        返回容器可以储存的最大水量。
   说明：你不能倾斜容器。

   题解:
       容器能装水的高度由 短的柱子决定，因为水会从短边溢出
       容器能装水的宽度是两根柱子之间的距离，即 right - left
       面积 = 宽度 × 高度 = 距离 × 两根柱子较短的高度

       使用双指针, left = 0. right = n - 1
       盛水容器面积 = (right -left) * min(height[left], height[right])
       如果 height[left] < height[right], 左移 left++, 否则 right--
*/
pub fn max_area(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let n = height.len();
    let (mut left, mut right) = (0, n - 1);

    let mut result: usize = 0;
    while left < right {
        let res = (right - left) * (min(height[left], height[right]) as usize);
        result = max(result, res);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    result as i32
}
