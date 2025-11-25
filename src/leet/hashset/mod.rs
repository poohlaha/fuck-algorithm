use std::collections::HashSet;

pub mod test;

/**
   2154. 将找到的值乘以 2
   力扣: https://leetcode.cn/problems/keep-multiplying-found-values-by-two/description/
   题目: 给你一个整数数组 nums ，另给你一个整数 original ，这是需要在 nums 中搜索的第一个数字。
        接下来，你需要按下述步骤操作:
        1. 如果在 nums 中找到 original ，将 original 乘以 2 ，得到新 original（即，令 original = 2 * original）。
        2. 否则，停止这一过程。
        3. 只要能在数组中找到新 original ，就对新 original 继续 重复 这一过程。
        返回 original 的 最终 值。

   时间复杂度: O(n)
   空间复杂度: O(n)
*/
pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut original = original;
    let set: HashSet<i32> = nums.into_iter().collect();
    while set.contains(&original) {
        original *= 2;
    }

    original
}
