/**
  1011. 在 D 天内送达包裹的能力
  力扣: https://leetcode.cn/problems/capacity-to-ship-packages-within-d-days/description/?envType=problem-list-v2&envId=59jEaTgw
  题目: 传送带上的包裹必须在 days 天内从一个港口运送到另一个港口。
       传送带上的第 i 个包裹的重量为 weights[i]。每一天，我们都会按给出重量（weights）的顺序往传送带上装载包裹。我们装载的重量不会超过船的最大运载重量。
       返回能在 days 天内将传送带上的所有包裹送达的船的最低运载能力。

  思路: 二分查找 + 贪心
  解:
    weights = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], days = 5
    初始化
      确定搜索范围
        最小可能载重 = 不能小于最大包裹 ⇒ 10
        最大可能载重 = 一天全运 ⇒ 55

    步骤:
        1. 查找 mid = (10 + 55) / 2 = 32
           判断载重 32 能否在 5 天内完成:

           Day1:
           1 + 2 + 3 + 4 + 5 + 6 + 7 = 28
           再加 8 会超过 32 → 结束

           Day2:
           8 + 9 = 17
           再加 10 会超过 32 → 结束

           Day3:
           10 → 结束

           总共用了 3 天 ≤ 5 → 32 可行
           → 缩小范围: 高位改为 32
           → 范围: [10, 32]

        2. 查找 mid = (10 + 32) / 2 = 21
           判断载重 21 能否在 5 天内完成:

           Day1:
           1 + 2 + 3 + 4 + 5 + 6 = 21
           再加 7 会超过 21 → 结束

           Day2:
           7 + 8 = 15
           再加 9 会超过 21 → 结束

           Day3:
           9 + 10 = 19 → 结束

           总共 3 天 ≤ 5 → 21 可行
           → 缩小范围: 高位改为 21
           → 范围: [10, 21]

        3. 查找 mid = (10 + 21) / 2 = 15
           判断载重 15 能否在 5 天内完成:

           Day1:
           1 + 2 + 3 + 4 + 5 = 15 → 结束

           Day2:
           6 + 7 = 13
           再加 8 会超过 15 → 结束

           Day3:
           8 → 结束

           Day4:
           9 → 结束

           Day5:
           10 → 结束

           总共刚好 5 天 ≤ 5 → 15 可行
           → 缩小范围: 高位改为 15
           → 范围: [10, 15]

        4. 查找 mid = (10 + 15) / 2 = 12
           判断载重 12 能否在 5 天内完成:

          Day1:
           1 + 2 + 3 + 4 = 10
           再加 5 会超过 12 → 结束

           Day2:
           5 + 6 = 11
           再加 7 会超过 12 → 结束

           Day3:
           7 → 结束

           Day4:
           8 → 结束

           Day5:
           9 → 结束

           Day6:
           10 → 结束

           总共 6 天 > 5 → 12 不可行
           → 载重太小
           → 范围: [13, 15]

        5. 查找 mid = (13 + 15) / 2 = 14
           判断载重 14 能否在 5 天内完成:

           Day1:
           1 + 2 + 3 + 4 = 10
           再加 5 会超过 14 → 结束

           Day2:
           5 + 6 = 11
           再加 7 会超过 14 → 结束

           Day3:
           7 → 结束

           Day4:
           8 → 结束

           Day5:
           9 → 结束

           Day6:
           10 → 结束

           总共 6 天 > 5 → 14 不可行
           → 缩小范围: 低位改为 mid + 1 =  15
           → 范围: [15, 15]

          最终答案: 15
*/
pub struct Ship;

impl Ship {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        if weights.len() == 0 {
            return 0;
        }

        if days == 0 {
            return 0;
        }

        // 获取最大值
        let mut max = weights.iter().max().unwrap_or(&0).clone();
        if max == 0 {
            return 0;
        }

        // 获取总重量
        let mut sum = weights.iter().sum();
        if sum == 0 {
            return 0;
        }

        let mut left = max;
        let mut right = sum;

        // 二分搜索最小可行载重
        while left < right {
            let mid = left + (right - left) / 2;

            if Self::can_ship(&weights, days, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    // 判断载重 capacity 能否在 days 天内把包裹按顺序送完
    fn can_ship(weights: &Vec<i32>, days: i32, capacity: i32) -> bool {
        let mut days_used = 1;
        let mut curr = 0;

        for &w in weights {
            if curr + w <= capacity {
                curr += w;
            } else {
                days_used += 1;
                curr = w;
                if days_used > days {
                    return false;
                }
            }
        }

        true
    }
}
