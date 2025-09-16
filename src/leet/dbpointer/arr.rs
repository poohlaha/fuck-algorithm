/**
  26. 删除有序数组中的重复项
  力扣: https://leetcode.cn/problems/remove-duplicates-from-sorted-array/
  题目: 给你一个 非严格递增排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。然后返回 nums 中唯一元素的个数。
       考虑 nums 的唯一元素的数量为 k ，你需要做以下事情确保你的题解可以被通过：
       更改数组 nums ，使 nums 的前 k 个元素包含唯一元素，并按照它们最初在 nums 中出现的顺序排列。nums 的其余元素与 nums 的大小不重要。
       返回 k

  解:
     使用原地算法 + 双指针(快慢指针)
     时间复杂度: O(n)
     空间复杂度: O(1)
*/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    if nums.len() == 1 {
        return 1;
    }

    let n = nums.len();
    let mut p1 = 0; // 慢指针
    let mut p2 = 1; // 快指针

    while p2 < n {
        // 相同则跳过
        if nums[p1] == nums[p2] {
            p2 += 1;
            continue;
        }

        // 不相同, 把 p2 值写入到 p1 位置
        p1 += 1;
        // 把 arr[p2] 写到 arr[p1] 位置
        nums[p1] = nums[p2];
        p2 += 1;
    }

    (p1 + 1) as i32
}

/**
   27. 移除元素
   力扣: https://leetcode.cn/problems/remove-element/
   题目: 给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素。元素的顺序可能发生改变。然后返回 nums 中与 val 不同的元素的数量。
        假设 nums 中不等于 val 的元素数量为 k，要通过此题，您需要执行以下操作:
        更改 nums 数组，使 nums 的前 k 个元素包含不等于 val 的元素。nums 的其余元素和 nums 的大小并不重要。
        返回 k。
    解:
      使用原地算法 + 双指针(快慢指针)
      时间复杂度: O(n)(每个元素只扫描一次)
      空间复杂度: O(1)(只用了两个指针)
*/
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let n = nums.len();
    let mut p1 = 0; // 慢指针
    let mut p2 = 0; // 快指针

    while p2 < n {
        // 相同则跳过
        if nums[p2] == val {
            p2 += 1;
            continue;
        }

        // 把 arr[p2] 写到 arr[p1] 位置
        nums[p1] = nums[p2];
        p1 += 1;
        p2 += 1;
    }

    p1 as i32
}
