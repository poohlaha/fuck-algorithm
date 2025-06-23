/*!
 数组
*/

use std::collections::HashMap;

mod difference;
mod region;
pub mod test;

pub struct Array;

impl Array {
    /**
      两数之和II - 输入有序数组:
      地址: https://leetcode.cn/problems/two-sum-ii-input-array-is-sorted/
      问题: 给你一个下标从 1 开始的整数数组 numbers ，该数组已按 非递减顺序排列，请你从数组中找出满足相加之和等于目标数 target 的两个数。
           如果设这两个数分别是 numbers[index1] 和 numbers[index2]，则 1 <= index1 < index2 <= numbers.length 。
           以长度为 2 的整数数组 [index1, index2] 的形式返回这两个整数的下标 index1 和 index2
           你可以假设每个输入 `只对应唯一的答案`，而且你 `不可以` 重复使用相同的元素
            你所设计的解决方案必须只使用常量级的额外空间

      条件总结:
      1. 有序数组 -> 非递减, 即 `升序` 排列
      2. 返回两个数的下标(从 `1` 开始计数，不是从 `0`)
      3. 只用 `常量级空间` -> 不能使用哈希表或额外数组
      4. 只存在一个解 -> 不用考虑多个答案

     解:
       使用 `双指针法`
       1. 定义两个指针
          - left: 从头开始, 指向索引 `0`
          - right: 从末尾开始, 指向索引 `n - 1`
       2. 循环判断
          - 如果 `numbers[left] + numbers[right] == target`，返回 `[left + 1, right + 1]`
          - 如果和 `大于 target`, 说明右边的数大了 -> right--
          - 如果和 `小于 target`, 说明左边的数小了 -> left++
       3. 一定能找到结果，因为题目保证 `有且只有一组答案`

      时间复杂度: O(n), 每个元素最多访问一次
      空间复杂度: O(1), 只使用了常量级额外空间
    */
    fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return Vec::new();
        }

        if nums.len() == 1 {
            if nums[0] == target {
                return vec![0, 0];
            }

            return Vec::new();
        }

        // 1. 定义两个指针
        let mut left = 0;
        let mut right = nums.len() - 1;

        // 2. 循环判断
        while left < right {
            // 2.1 如果 `numbers[left] + numbers[right] == target`，返回 `[left + 1, right + 1]`
            if nums[left] + nums[right] == target {
                return vec![(left + 1) as i32, (right + 1) as i32];
            }

            // 2.2 如果和 `大于 target`, 说明右边的数大了 -> right--
            if nums[left] + nums[right] > target {
                right -= 1;
            }

            // 2.3 如果和 `小于 target`, 说明左边的数小了 -> left++
            if nums[left] + nums[right] < target {
                left += 1;
            }
        }

        Vec::new()
    }

    /**
     两数之和:
     地址: https://leetcode.cn/problems/two-sum/
     问题: 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
          你可以假设每种输入只会对应一个答案，并且你不能使用两次相同的元素。
          你可以按任意顺序返回答案
          时间复杂度小于 O(n2)

     1. 双指针
         - 时间复杂度: O(n²)
         - 空间复杂度: O(1)
     2. 哈希表
        - 时间复杂度: O(n) — 遍历一次数组
        - 空间复杂度: O(n) — 最坏情况需存所有元素进哈希表
    */
    // 哈希表
    pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return Vec::new();
        }

        if nums.len() == 1 {
            if nums[0] == target {
                return vec![0, 0];
            }

            return Vec::new();
        }

        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let com = target - num;
            if let Some(&j) = map.get(&com) {
                return vec![j as i32, i as i32];
            }

            map.insert(num, i);
        }

        Vec::new()
    }

    // 双指针, 每次从 left 向后查找
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return Vec::new();
        }

        if nums.len() == 1 {
            if nums[0] == target {
                return vec![0, 0];
            }

            return Vec::new();
        }

        let n = nums.len();
        let mut left = 0;
        while left < n {
            let mut right = left + 1;
            while right < n {
                if nums[left] + nums[right] == target {
                    return vec![left as i32, right as i32];
                }

                right += 1;
            }

            left += 1;
        }

        Vec::new()
    }

    /**
    三数之和:
    地址: https://leetcode-cn.com/problems/3sum/
    问题: 给你一个整数数组 nums，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。
         请你返回所有和为 0 且不重复的三元组
         答案中不可以包含重复的三元组
    条件总结:
    1. 三元组的三个索引不同，即 i != j != k
    2. 三元组的值和为0
    3. `不允许结果中有重复的三元组`，比如 [-2, 0, 2] 出现两次，算重复，需要去重
    4. `三元组中的元素可以重复`，比如两个 1

    解: 使用 `循环` 扣除后, 余下的用双指针
    */
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return Vec::new();
        }

        if nums.len() == 1 {
            if nums[0] == 0 {
                return vec![vec![nums[0]]];
            }

            return Vec::new();
        }

        let mut numbers = nums.clone();
        numbers.sort();

        let mut res = Vec::new();
        let target = 0;

        for i in 0..numbers.len() {
            // 已经排过序了, 后面只会更大
            if numbers[i] > 0 {
                break;
            }

            // 跳过重复元素
            if i > 0 && numbers[i] == numbers[i - 1] {
                continue;
            }

            let total = target - numbers[i];

            // 双指针
            let mut left = i + 1;
            let mut right = numbers.len() - 1;

            while left < right {
                let sum = numbers[left] + numbers[right];
                if sum == total {
                    res.push(vec![numbers[i], numbers[left], numbers[right]]);

                    left += 1;
                    right -= 1;

                    // 跳过相邻的重复元素(防止元素被重复加入)(用前判断)
                    while left < right && numbers[left] == numbers[left - 1] {
                        left += 1;
                    }

                    while left < right && numbers[right] == numbers[right + 1] {
                        right -= 1;
                    }
                }

                // 如果和 `小于 target`, 说明左边的数小了 -> left++
                if sum < total {
                    left += 1;
                }

                // 如果和 `大于 target`, 说明右边的数大了 -> right--
                if sum > total {
                    right -= 1;
                }
            }
        }

        res
    }

    /**
    四数之和:
    地址: https://leetcode.cn/problems/4sum/
    问题: 给你一个由 n 个整数组成的数组 nums ，和一个目标值 target。
         请你找出并返回满足下述全部条件且不重复的四元组 [nums[a], nums[b], nums[c], nums[d]] (若两个四元组元素一一对应，则认为两个四元组重复):
         0 <= a, b, c, d < n
         a、b、c 和 d 互不相同
         nums[a] + nums[b] + nums[c] + nums[d] == target
         你可以按 任意顺序 返回答案

    条件总结:
    1. 四个下标互不相同：a != b != c != d
    2. 四个数和等于 target：nums[a] + nums[b] + nums[c] + nums[d] == target
    3. 结果中不包含重复的四元组（元素相同即重复）

    解:
      1. 先排序数组(方便去重或剪枝)
      2. 用 `两层循环固定前两个数` nums[i] 和 nums[j]
      3. 然后对剩余部分使用双指针 left 和 right 查找满足 nums[left] + nums[right] == target - nums[i] - nums[j]
      4. 跳过重复元素避免重复结果
         - i: 如果 nums[i] == nums[i - 1]，跳过，避免重复四元组
         - j: 如果 nums[j] == nums[j - 1]，跳过，避免重复
         - 移动 left, 跳过和前一个相同的元素
         - 移动 right, 跳过和前一个相同的元素
      5. 提前终止循环(剪枝)
         - i:
            - 如果 nums[i] + nums[i+1] + nums[i+2] + nums[i+3] > target，说明最小四数和都超过目标，后面更大数无解，提前退出外层循环
            - 如果 nums[i] + nums[n-3] + nums[n-2] + nums[n-1] < target，说明最大四数和都小于目标，说明当前 i 不可能构成解, 继续下一 i
         - j
            - 如果 nums[i] + nums[j] + nums[j+1] + nums[j+2] > target，说明最小四数和都超过目标，后面更大数无解，提前退出内层循环
            - 如果 nums[i] + nums[j] + nums[n-1] + nums[n-2] < target，说明最大四数和都小于目标，说明当前 j 不可能构成解, 继续下一 j
    */
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n == 0 {
            return Vec::new();
        }

        if n < 4 {
            return Vec::new();
        }

        let mut numbers = nums.clone();
        numbers.sort();

        let total = target as i64;
        let mut res = Vec::new();
        for i in 0..n - 3 {
            //  剪枝条件 1: 如果最小4数和 > target，说明后续无解
            if (numbers[i] as i64)
                + (numbers[i + 1] as i64)
                + (numbers[i + 2] as i64)
                + (numbers[i + 3] as i64)
                > total
            {
                break;
            }

            //  剪枝条件 2: 如果最小4数和 < target，说明当前 i 不可能构成解, 继续下一 i
            if (numbers[i] as i64)
                + (numbers[n - 1] as i64)
                + (numbers[n - 2] as i64)
                + (numbers[n - 3] as i64)
                < total
            {
                continue;
            }

            // 跳过重复的值
            if i > 0 && (numbers[i] as i64) == (numbers[i - 1] as i64) {
                continue;
            }

            // 内层循环
            // i < j < left < right
            for j in i + 1..n - 2 {
                //  剪枝条件 1: 如果最小4数和 > target，说明后续无解
                if (numbers[i] as i64)
                    + (numbers[j] as i64)
                    + (numbers[j + 1] as i64)
                    + (numbers[j + 2] as i64)
                    > total
                {
                    break;
                }

                // 剪枝条件 2: 如果最小4数和 < target，说明当前 j 不可能构成解, 继续下一 j
                if (numbers[i] as i64)
                    + (numbers[j] as i64)
                    + (numbers[n - 1] as i64)
                    + (numbers[n - 2] as i64)
                    < total
                {
                    continue;
                }

                // 跳过重复的值
                if j > i + 1 && (numbers[j] as i64) == (numbers[j - 1] as i64) {
                    continue;
                }

                // 双指针查找
                let mut left = j + 1;
                let mut right = n - 1;

                while left < right {
                    let sum = (numbers[i] as i64)
                        + (numbers[j] as i64)
                        + (numbers[left] as i64)
                        + (numbers[right] as i64);
                    if sum == total {
                        res.push(vec![numbers[i], numbers[j], numbers[left], numbers[right]]);

                        // 跳过重复的值(用完再跳)
                        while left < right && (numbers[left] as i64) == (numbers[left + 1] as i64) {
                            left += 1;
                        }

                        while left < right && (numbers[right] as i64) == (numbers[right - 1] as i64)
                        {
                            right -= 1;
                        }

                        left += 1;
                        right -= 1;
                    }

                    // 如果和 `小于 target`, 说明左边的数小了 -> left++
                    if sum < total {
                        left += 1;
                    }

                    // 如果和 `大于 target`, 说明右边的数大了 -> right--
                    if sum > total {
                        right -= 1;
                    }
                }
            }
        }

        res
    }
}
