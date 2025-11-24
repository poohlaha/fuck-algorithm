/**
   34. 在排序数组中查找元素的第一个和最后一个位置
   力扣: https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/description/
   题目: 给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。请你找出给定目标值在数组中的开始位置和结束位置。
        如果数组中不存在目标值 target，返回 [-1, -1]。
        你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。

   解: 使用两次二分查找算法, 第一次查找第一个位置, 第二次查找第二个位置

   时间复杂度: O(log n)
   空间复杂度: O(1)
*/
pub struct Range;

impl Range {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let first = Self::first(&nums, target);
        if first == -1 {
            return vec![-1, -1];
        }

        let last = Self::last(&nums, target);

        vec![first, last]
    }

    fn first(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = (nums.len()) as i32 - 1;
        let mut answer = -1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] >= target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }

            if nums[mid as usize] == target {
                answer = mid;
            }
        }

        answer
    }

    fn last(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = (nums.len()) as i32 - 1;
        let mut answer = -1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] <= target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }

            if nums[mid as usize] == target {
                answer = mid;
            }
        }

        answer
    }
}
