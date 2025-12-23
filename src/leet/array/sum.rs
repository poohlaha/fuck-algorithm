/**
   3381. 长度可被 K 整除的子数组的最大元素和
   力扣: https://leetcode.cn/problems/maximum-subarray-sum-with-length-divisible-by-k/description
   题目: 给你一个整数数组 nums 和一个整数 k 。
        返回 nums 中一个 非空子数组 的 最大和，要求该子数组的长度可以 被 k 整除。

   思路: 使用 前缀和
   解: (r + 1 - l) % k == 0  => (r + 1) % k == l % k

      前缀和(长度 n + 1): prefix[i] = A[0..i-1] 的和
      子数组A[l .. r] 的和: sum = prefix[r+1] - prefix[l]

      子数组长度：
        len = (r - l + 1)
            = (r + 1) - l
            = index2 - index1
        即: 两个 prefix 下标之间的差，就是子数组的长度

      要求: 子数组长度能被 k 整除
        (index2 - index1) % k == 0
         index2 % k == index1 % k
      即: 两个前缀和下标对 k 取模相同，它们的差一定是 k 的倍数

      例:
        k = 3
        prefix[5] % 3 = 2
        prefix[2] % 3 = 2
        -> (5 - 2) % 3 = 0
        -> 它们对应的子数组长度 = 3 的倍数
        -> 所以它们之间的子数组满足要求

      前缀和下标 i 处: sum = prefix[i] - prefix[j]
      i 范围: [l = j, r = i - 1]

      `best[m] = 最小前缀和` 改成 `best[m] = (最小前缀和, 下标)`

      分组: m = index % k
      只有落在同一“模 k 类别”的前缀下标之间，差值才能是 k 的倍数。

      例:
        k = 3
        所有下标对:
        0 --- 3 --- 6 --- 9  ...
        1 --- 4 --- 7 --- 10 ...
        2 --- 5 --- 8 --- 11 ...
        发现: 两点之间的距离只可能是 3 的倍数, 因为: 这些点的下标 % 3 都相等

        下标: 0 1 2 3 4 5 6 7 8
        % 3: 0 1 2 0 1 2 0 1 2

        现在想找长度是 3 的倍数的子数组:
        只能从:
          0 → 3
          0 → 6
          1 → 4
          1 → 7
          2 → 5
          2 → 8
        因为:
          (0 % 3) == (3 % 3)
          (1 % 3) == (4 % 3)
          (2 % 3) == (5 % 3)
          ...

        所以这样分组:
          组 0：所有 `i % 3 == 0` 的前缀下标
          组 1：所有 `i % 3 == 1` 的前缀下标
          组 2：所有 `i % 3 == 2` 的前缀下标
        组内任何两个下标 i、j 满足: `(i - j) % 3 = 0`, 即: 组内任意两点之间的区间长度都是 3 的倍数。
        所以用 `i % k` 来分组, 每个组只存一个值: 组内遇到过的最小前缀和及其下标

      口诀: 按 i % k 分组 → 每组只保留最小 prefix → 遍历 i 计算 sum → 最大 sum 自动产生

      步骤:
         1. 先求前缀和 p，长度 n+1。
         2. 遍历每个前缀下标 i，并按 i % k 对前缀和进行分组。
         3. 每组只保存“目前组内最小前缀和和它的下标”。
         4. 对于每个 i，若该组已有前缀 j:
            sum = p[i] - p[j]  // 子数组和
            (因为 i%k == j%k -> 子数组长度 % k == 0)
         5. 当 sum > max_sum 时更新最大和以及左右边界。
         6. 若当前 p[i] 是该组内历史最小前缀和，则更新 best[m]。

      时间复杂度: O(n)
      空间复杂度: O(k)
*/
pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    if nums.is_empty() {
        return 0;
    }

    let n = nums.len();

    // 前缀和
    let mut p = vec![0i64; n + 1];
    for i in 0..n {
        p[i + 1] = p[i] + nums[i] as i64;
    }

    // (最小前缀和, 下标), 每个组只存一个值: 组内遇到过的最小前缀和及其下标
    let mut best = vec![(i64::MAX, 0usize); k as usize];
    let mut max_sum = i64::MIN;
    let mut l = 0usize;
    let mut r = 0usize;

    for i in 0..=n {
        // 用 `i % k` 来分组
        let m = (i as i32 % k) as usize;

        if best[m].0 != i64::MAX {
            let (min_prefix, idx) = best[m];
            // nums[idx..i-1], sum = p[i] - p[idx], 同一组的最小前缀和相减
            let sum = p[i] - min_prefix;

            if sum > max_sum {
                max_sum = sum;
                l = idx; // 子数组左端点
                r = i - 1; // 子数组右端点
            }
        }

        // 更新数组的最小前缀和
        // sum = p[i] - best[m].0
        if p[i] < best[m].0 {
            best[m] = (p[i], i)
        }
    }

    // 长度: (r - l + 1) as i64
    max_sum
}
