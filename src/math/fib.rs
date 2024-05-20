//! 斐波那契数列

/**
斐波那契数列是指这样一个数列：1、1、2、3、5、8、13、21、34、……在数学上，斐波那契数列以递归的方法定义：
F{1} = 1, F{2} = 1, F(n) = F(n - 1) + F(n - 2)(n >= 3)
斐波那契数列中的每个数等于其前两个数的和，而第一个和第二个数均为 1

自顶而下:
 - 使用递归的方式解决问题，通常从问题的顶部开始，逐步向下解决子问题，直到达到基本情况（边界条件）。
 - 递归过程中，通常会使用记忆化搜索（Memoization）来避免重复计算

自底向上:
- 使用迭代的方式解决问题，通常从最小的子问题开始，逐步计算出所有的子问题的解，直到达到目标问题的解。
- 通常不使用递归，而是使用循环来迭代计算。
**/

/// 暴力递归
pub(crate) fn fib(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}

/// 带备忘录的递归解法(自顶向下)
/**
带「备忘录」的递归算法，把一棵存在巨量冗余的递归树通过「剪枝」，改造成了一幅不存在冗余的递归图，极大减少了子问题（即递归图中节点）的个数。
*/
pub(crate) fn dp_fib(n: u32) -> u32 {
    let max = n as usize;
    let mut memo = vec![0u32; max + 1]; // 斐波那契数列的长度为 n 时，数组 memo 中应该包含 n 个元素来存储每个位置的斐波那契数值

    fn dp(memo: &mut Vec<u32>, n: u32) -> u32 {
        if n == 0 || n == 1 {
            return n;
        }

        if memo[n as usize] != 0 {
            return memo[n as usize];
        }

        memo[n as usize] = dp(memo, n - 1) + dp(memo, n - 2);
        memo[n as usize]
    }

    dp(&mut memo, n)
}

/// dp 数组的迭代（递推）解法(自底而上)
pub(crate) fn db_cycle_fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    let max = n as usize;
    let mut memo = vec![0u32; max + 1]; // 斐波那契数列的长度为 n 时，数组 memo 中应该包含 n 个元素来存储每个位置的斐波那契数值
    memo[0] = 1;
    memo[1] = 1;

    for i in 2..=n {
        let x1 = i - 1;
        let x2 = i - 2;
        memo[i as usize] = memo[x1 as usize] + memo[x2 as usize];
    }

    memo[(n - 1) as usize]
}

/// 常用, 优化空间复杂度为O(1)
pub(crate) fn db_normal_fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    // 从 0 开始
    let mut dp_i_1 = 1;
    let mut dp_i_2 = 0;
    for i in 2..=n {
        let dp_i = dp_i_1 + dp_i_2;
        dp_i_2 = dp_i_1;
        dp_i_1 = dp_i
    }

    dp_i_1
}

/// 零钱兑换(暴力递归)
/**
给你 k 种面值的硬币，面值分别为 c1, c2 ... ck，每种硬币的数量无限，再给一个总金额 amount，问你最少需要几枚硬币凑出这个金额
1、确定 base case，这个很简单，显然目标金额 amount 为 0 时算法返回 0，因为不需要任何硬币就已经凑出目标金额了。
2、确定「状态」，也就是原问题和子问题中会变化的变量。由于硬币数量无限，硬币的面额也是题目给定的，只有目标金额会不断地向 base case 靠近，所以唯一的「状态」就是目标金额 amount。
3、确定「选择」，也就是导致「状态」产生变化的行为。目标金额为什么变化呢，因为你在选择硬币，你每选择一枚硬币，就相当于减少了目标金额。所以说所有硬币的面值，就是你的「选择」。
4、明确 dp 函数/数组的定义。我们这里讲的是自顶向下的解法，所以会有一个递归的 dp 函数，一般来说函数的参数就是状态转移中会变化的量，也就是上面说到的「状态」；函数的返回值就是题目要求我们计算的量。就本题来说，状态只有一个，即「目标金额」，题目要求我们计算凑出目标金额所需的最少硬币数量。
所以我们可以这样定义 dp 函数：dp(n) 表示，输入一个目标金额 n，返回凑出目标金额 n 所需的最少硬币数量。
*/
pub(crate) fn coin_change(coins: &Vec<u32>, amount: i32) -> i32 {
    if coins.is_empty() {
        return -1;
    }

    // base case
    if amount == 0 {
        return 0;
    }

    if amount < 0 {
        return -1;
    }

    let mut result = i32::MAX;
    for coin in coins.iter() {
        let coin = *coin as i32;

        // 计算子问题的结果
        let res = coin_change(coins, amount - coin);

        // 子问题无解则跳过
        if res == -1 {
            continue;
        }

        result = std::cmp::min(result, res + 1); // 选择 1 个硬币尝试
    }

    return if result == i32::MAX { -1 } else { result };
}

/// 零钱兑换(带备忘录的递归)
pub(crate) fn dp_coin_change(coins: &Vec<u32>, amount: i32) -> i32 {
    if coins.is_empty() {
        return -1;
    }

    let max = amount as usize;
    let mut memo = vec![-666i32; max + 1];

    fn dp(coins: &Vec<u32>, amount: i32, memo: &mut Vec<i32>) -> i32 {
        // base case
        if amount == 0 {
            return 0;
        }

        if amount < 0 {
            return -1;
        }

        if memo[amount as usize] != -666 {
            return memo[amount as usize];
        }

        let mut result = i32::MAX;
        for coin in coins.iter() {
            let coin = *coin as i32;

            // 计算子问题的结果
            let res = coin_change(coins, amount - coin);

            // 子问题无解则跳过
            if res == -1 {
                continue;
            }

            result = std::cmp::min(result, res + 1); // 选择 1 个硬币尝试
        }

        memo[amount as usize] = if result == i32::MAX { -1 } else { result };
        return memo[amount as usize];
    }

    dp(coins, amount, &mut memo)
}

/// 零钱兑换(dp 数组的迭代解法, 自底向上)
pub(crate) fn db_cycle_coin_change(coins: &Vec<u32>, amount: i32) -> i32 {
    if coins.is_empty() {
        return -1;
    }

    let max = amount as usize;
    let mut memo = vec![amount + 1; max + 1];

    memo[0] = 0;

    for i in 0..memo.len() {
        for coin in coins.iter() {
            let v = i as i32 - *coin as i32; // 计算减去当前硬币面额后的金额
            if v < 0 {
                continue;
            }

            memo[i] = std::cmp::min(memo[i], memo[v as usize] + 1);
        }
    }

    return if memo[amount as usize] == -amount + 1 {
        -1
    } else {
        memo[amount as usize]
    };
}

/// 最长递增子序列, 动态规划解法, 时间复杂度 O(N^2)
/// 其实最长递增子序列和一种叫做 patience game 的纸牌游戏有关，甚至有一种排序方法就叫做 patience sorting（耐心排序）
pub(crate) fn length_of_lis(v: Vec<u32>) -> i32 {
    if v.is_empty() {
        return 0;
    }

    let max = v.len();
    let mut memo = vec![1; max];

    for i in 0..max {
        for j in 0..i {
            if v[j] < v[i] {
                memo[i] = std::cmp::max(memo[i], memo[j] + 1)
            }
        }
    }

    let mut result = 0;
    for i in 0..memo.len() {
        result = std::cmp::max(result, memo[i]);
    }

    result
}

/// 最长递增子序列, 二分查找解法, 时间复杂度为 O(NlogN)
/**
`  1. 只能把点数小的牌压到点数比它大的牌上
   2. 如果当前牌点数较大没有可以放置的堆，则新建一个堆，把这张牌放进去
   3. 如果当前牌有多个堆可供选择，则选择最左边的那一堆放置
*/
pub(crate) fn length_of_lis_with_two(v: Vec<u32>) -> i32 {
    if v.is_empty() {
        return 0;
    }

    let max = v.len();
    let mut memo = vec![0; max];
    let mut piles: usize = 0; // 牌堆数初始化为 0
    for i in 0..max {
        let poker = v[i]; // 要处理的扑克牌

        // 二分查找
        let mut left: usize = 0;
        let mut right: usize = piles;
        while left < right {
            // 向下取整
            let middle = left + (right - left) / 2; // left + [left, right] 的中位数
            if poker <= memo[middle] {
                right = middle; // poker <= 最后一个数大于, 则替代最后一个数
            } else if poker > memo[middle] {
                left = middle + 1; // poker > 最后一个数大于, 则添加到最后
            }
        }

        // 没找到合适的牌堆，新建一堆
        if left == piles {
            piles += 1;
        }

        // 把这张牌放到牌堆顶
        memo[left] = poker;
    }

    piles as i32
}

/// 俄罗斯套娃信封问题, 时间复杂度为 O(NlogN)
/**
给定一组信封的宽度和高度对(𝑤,ℎ),求最大的嵌套序列长度
1. 先对宽度 w 进行升序排序，如果遇到 w 相同的情况，则按照高度 h 降序排序
2. 之后把所有的 h 作为一个数组，在这个数组上计算 LIS 的长度
*/
pub(crate) fn max_envelopes(v: Vec<(u32, u32)>) -> i32 {
    if v.is_empty() {
        return 0;
    }

    let mut envelopes = v;
    envelopes.sort_by(|a, b| {
        if a.0 == b.0 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let heights: Vec<u32> = envelopes.iter().map(|&(_, h)| h).collect();
    return length_of_lis_with_two(heights);
}

/// 最大子数组和
/**
  使用滑动窗口算法, 当当前累加值 < 0 时, 收缩窗口
**/
pub(crate) fn max_sub_array(v: Vec<i32>) -> i32 {
    if v.is_empty() {
        return -1;
    }

    let mut left = 0;
    let mut right = 0;
    let mut current_sum = 0;
    let mut max_sum = i32::MIN;
    while right < v.len() {
        current_sum += v[right];
        max_sum = std::cmp::max(max_sum, current_sum);
        right += 1;

        while current_sum < 0 {
            current_sum -= v[left];
            left += 1;
        }
    }

    max_sum
}

/// 最大子数组和
/**
 使用 Kadane算法(卡丹算法或卡丹尼算法), 动态规划算法的一种
 1. 设置 current_num 和 max_sum 为数组的第一个元素
 2. 遍历数组, current_num 取 每一项值 和 current_num + 每一项值的累计值中的最大值, max_sum 取其和 current_num 中的最大值
*/
pub(crate) fn max_sub_array_kadane(v: Vec<i32>) -> i32 {
    if v.is_empty() {
        return -1;
    }

    let mut current_num = v[0];
    let mut max_sum = v[0];

    for i in v.iter().skip(1) {
        let v = i.clone();
        current_num = std::cmp::max(v, current_num + v);
        max_sum = std::cmp::max(max_sum, current_num);
    }

    max_sum
}

/// 最大子数组和
/**
  使用动态规划算法
  1. 确定 base case
    dp[0] 为第一个元素
  2. 确定 `状态`，也就是原问题和子问题中会变化的变量
    用 dp[i] 表示以 nums[i] 结尾的子数组的最大和
  3. 确定 `选择`，也就是导致 `状态` 产生变化的行为
    dp[i] = max(nums[i], dp[i-1] + nums[i])。即对于每一个元素，决定是将其包含在当前子数组中（dp[i-1] + nums[i]），还是从它开始一个新的子数组（nums[i]）。
  4. 明确 `dp 函数/数组` 的定义
    dp = vec![-1;max]
  结果: 整个数组的最大子数组和为 max(dp[0], dp[1], ..., dp[n-1])。
*/
pub(crate) fn max_sub_array_dynamic(v: Vec<i32>) -> i32 {
    if v.is_empty() {
        return -1;
    }

    let max = v.len();
    let mut dp = vec![-1; max];

    // 1. base case
    dp[0] = v[0];

    // 状态转移方程
    for (i, _) in v.iter().skip(1).enumerate() {
        let index = i + 1;
        dp[index] = std::cmp::max(v[index], v[index] + dp[index - 1]);
    }

    let mut res = i32::MIN;
    for i in dp.iter() {
        res = std::cmp::max(res, i.clone());
    }

    res
}
