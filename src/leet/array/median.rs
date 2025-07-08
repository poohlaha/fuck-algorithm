/**
  4. 寻找两个正序数组的中位数
  力扣: https://leetcode.cn/problems/median-of-two-sorted-arrays/description/
  题目: 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数
       算法的时间复杂度应该为 O(log (m+n))

  解:
  只要满足:
      L1 <= R2   （A 左边最大 ≤ B 右边最小）
      L2 <= R1   （B 左边最大 ≤ A 右边最小）
  则取: max(L1, L2)
*/
use std::cmp::{max, min};
use std::mem::swap;

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (mut m, mut n) = (nums1.len(), nums2.len());
    if m == 0 && n == 0 {
        return 0.0;
    }

    let (mut nums_a, mut nums_b) = (nums1, nums2);

    nums_a.sort();
    nums_b.sort();

    if m > n {
        // 交换 m 和 n, 交换 nums_a 和 nums_b
        swap(&mut m, &mut n);
        swap(&mut nums_a, &mut nums_b);
    }

    let total = m + n;

    // 计算切分点: i + j = (total + 1) / 2
    let half = (total + 1) / 2;

    let mut left: isize = 0;
    let mut right: isize = m as isize;

    while left <= right {
        let i = ((left + right) / 2) as usize;
        let j = half as isize - i as isize;

        let l1 = if i == 0 { i32::MIN } else { nums_a[i - 1] };
        let r1 = if i == m { i32::MAX } else { nums_a[i] };
        let l2 = if j == 0 { i32::MIN } else { nums_b[(j - 1) as usize] };
        let r2 = if j == n as isize { i32::MAX } else { nums_b[j as usize] };

        if l1 <= r2 && l2 <= r1 {
            let left_max = if i == 0 {
                l2
            } else if j == 0 {
                l1
            } else {
                max(l1, l2)
            };

            let right_min = if i == m {
                r2
            } else if j == n as isize {
                r1
            } else {
                min(r1, r2)
            };
            // 满足条件
            if total % 2 == 0 {
                // 偶数: 中位数 = (max(L1, L2) + min(R1, R2)) / 2
                return (left_max as f64 + right_min as f64) / 2.0;
            }

            // 奇数: 中位数 = max(L1, L2)
            return left_max as f64;
        }

        // 检查 L1 <= R2 && L2 <= R1
        // 左边切多了, 要减小，让右半部分变大
        if l1 > r2 {
            right = i as isize - 1;
        } else {
            // A 的右半部分太小，应该“再切更多 A 的元素放入左半部分”，让右半部分变大
            left = i as isize + 1;
        }
    }

    0.0
}
