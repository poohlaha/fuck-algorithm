/*!
    303. 区域和检索-数组不可变
    地址: https://leetcode.cn/problems/range-sum-query-immutable/description/
    问题: 给定一个整数数组  nums，处理以下类型的多个查询
         1. 计算索引 `left` 和 `right`(包含 `left` 和 `right`) 之间的 `nums` 元素的 `和`, 其中 `left` <= `right`
         实现 `NumArray` 类:
         `NumArray(int[] nums)` 使用数组 `nums` 初始化对象
         `int sumRange(int i, int j)` 返回数组 `nums` 中索引 `left` 和 `right` 之间的元素的 `总和` ，包含 `left` 和 `right` 两点（也就是 `nums[left] + nums[left + 1] + ... + nums[right]`)
     解:
       使用前缀和
       sum(i, j) = P[j] - P[i - 1] (i > 0)
       sum(0, j) = P[j]            (i = 0)

     时间复杂度
      - 构造前缀和: O(n)
      - 区间求和查询: O(1)
*/

pub struct NumArray {
    prefix: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        if nums.is_empty() {
            return Self { prefix: Vec::new() };
        }

        // 长度设计为 n + 1, 第一位是 0
        let mut prefix = vec![0; nums.len() + 1];
        for i in 1..=nums.len() {
            prefix[i] = prefix[i - 1] + nums[i - 1];
        }

        Self { prefix }
    }

    // 查询 [left, right]
    // prefix[right + 1] - prefix[left + 1 - 1] = prefix[right + 1] - prefix[left]
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        if self.prefix.is_empty() {
            return 0;
        }

        if left < 0 || right < 0 {
            return 0;
        }

        if left == 0 && right == 0 {
            return self.prefix[1];
        }

        // sum(0, j) = P[j] (i = 0)
        if left == 0 && right > 0 {
            return self.prefix[(right + 1) as usize];
        }

        let (l, r) = if left > right {
            (right, left)
        } else {
            (left, right)
        };

        // prefix[right + 1] - prefix[left + 1 - 1] = prefix[right + 1] - prefix[left]
        self.prefix[(r + 1) as usize] - self.prefix[l as usize]
    }
}
