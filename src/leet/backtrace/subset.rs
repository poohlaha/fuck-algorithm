/**
  2044. 统计按位或能得到最大值的子集数目
  力扣: https://leetcode.cn/problems/count-number-of-maximum-bitwise-or-subsets/description
  题目: 给你一个整数数组 nums ，请你找出 nums 子集 按位或 可能得到的 最大值 ，并返回按位或能得到最大值的 不同非空子集的数目 。
       如果数组 a 可以由数组 b 删除一些元素（或不删除）得到，则认为数组 a 是数组 b 的一个 子集 。如果选中的元素下标位置不一样，则认为两个子集 不同 。
       对数组 a 执行 按位或 ，结果等于 a[0] OR a[1] OR ... OR a[a.length - 1]（下标从 0 开始）。

       示例 1:
         输入: nums = [3, 1]
         输出: 2
         解释: 子集按位或能得到的最大值是 3 。有 2 个子集按位或可以得到 3:
              - [3]
              - [3, 1]

         示例2:
         输入: nums = [2, 2,2 ]
         输出: 7
         解释: [2, 2, 2] 的所有非空子集的按位或都可以得到 2 。总共有 23 - 1 = 7 个子集


         示例 3:
         输入: nums = [3, 2, 1, 5]
         输出: 6
         解释: 子集按位或可能的最大值是 7 。有 6 个子集按位或可以得到 7:
              - [3, 5]
              - [3, 1, 5]
              - [3, 2, 5]
              - [3, 2, 1, 5]
              - [2, 5]
              - [2, 1, 5]

   解: 使用 `回溯` 算法
       1. 当前状态(state)
          当前状态 = (index, cur_or)
          - index: 当前处理到 nums 的第几个元素
          - cur_or: 到目前为止选中的元素 OR 的结果

       2. 可选选择(choice)
          对子集问题来说，每个位置只有两个选择:
          choice 1: 不选 nums[index]
          choice 2: 选 nums[index]

       3. 约束条件(终止条件)
          if index == nums.len()
          说明所有元素都考虑完了, 此时:
          - 如果 `cur_or > 0` (非空子集)
          - 用它更新 `mar_or` 和 `count`

       4. 剪枝
          本题不需要 `剪枝`, 因为:
          -  n <= 16
          -  2ⁿ = 65536，完全可接受

       5. 撤销选择
          本题 `不需要显式撤销选择`
          因为:
          - cur_or 是 `值传递`
          - 每次递归传的是 新的 `cur_or`
          - 没有共享可变状态

          所以:
          ```
           做选择 + 撤销选择
                 ↓
           直接在递归参数里体现
          ```
     时间复杂度: O(2^n)
     空间复杂度: O(n)
*/
pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut max_or = 0;
    let mut count = 0;

    fn backtrack(nums: &Vec<i32>, index: usize, cur_or: i32, max_or: &mut i32, count: &mut i32) {
        if index == nums.len() {
            if cur_or == 0 {
                return; // 空子集, 跳过
            }

            if cur_or > *max_or {
                *max_or = cur_or;
                *count = 1; // 前面子集, 全部作废
            } else if cur_or == *max_or {
                *count += 1;
            }

            return;
        }

        // 不选 nums[index]
        backtrack(&nums, index + 1, cur_or, max_or, count);

        // 选 nums[index]
        backtrack(nums, index + 1, cur_or | nums[index], max_or, count);
    }

    backtrack(&nums, 0, 0, &mut max_or, &mut count);

    count
}
