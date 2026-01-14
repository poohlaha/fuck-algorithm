/*!
  3562. 折扣价交易股票的最大利润
  力扣: https://leetcode.cn/problems/maximum-profit-from-trading-stocks-with-discounts/description
  题目: 给你一个整数 n，表示公司中员工的数量。每位员工都分配了一个从 1 到 n 的唯一 ID ，其中员工 1 是 CEO。另给你两个下标从 1 开始的整数数组 present 和 future，两个数组的长度均为 n，具体定义如下：
       - present[i] 表示第 i 位员工今天可以购买股票的 当前价格 。
       - future[i] 表示第 i 位员工明天可以卖出股票的 预期价格 。
       公司的层级关系由二维整数数组 hierarchy 表示，其中 hierarchy[i] = [ui, vi] 表示员工 ui 是员工 vi 的直属上司。
       此外，再给你一个整数 budget，表示可用于投资的总预算。
       公司有一项折扣政策：如果某位员工的直属上司购买了自己的股票，那么该员工可以以 半价 购买自己的股票（即 floor(present[v] / 2)）。
       请返回在不超过给定预算的情况下可以获得的 最大利润。

   注意:
       - 每只股票最多只能购买一次。
       - 不能使用股票未来的收益来增加投资预算，购买只能依赖于 budget。

   示例:
       1. 输入: n = 2, present = [1,2], future = [4,3], hierarchy = [[1,2]], budget = 3
          输出: 5
          解释: 1 → 2
               - 员工 1 以价格 1 购买股票，获得利润 4 - 1 = 3
               - 由于员工 1 是员工 2 的直属上司，员工 2 可以以折扣价 floor(2 / 2) = 1 购买股票
               - 员工 2 以价格 1 购买股票，获得利润 3 - 1 = 2
               - 总购买成本为 1 + 1 = 2 <= budget，因此最大总利润为 3 + 2 = 5

        2. 输入: n = 2, present = [3,4], future = [5,8], hierarchy = [[1,2]], budget = 4
           输出: 4
           解释: 1 → 2
                - 员工 2 以价格 4 购买股票，获得利润 8 - 4 = 4
                - 由于两位员工无法同时购买，最大利润为 4

        3. 输入: n = 3, present = [4,6,8], future = [7,9,11], hierarchy = [[1,2],[1,3]], budget = 10
           输出: 10
           解释:    1
                  / \
                 3   2
               - 员工 1 以价格 4 购买股票，获得利润 7 - 4 = 3
               - 员工 3 可获得折扣价 floor(8 / 2) = 4，获得利润 11 - 4 = 7
               - 员工 1 和员工 3 的总购买成本为 4 + 4 = 8 <= budget，因此最大总利润为 3 + 7 = 10
        4. 输入: n = 3, present = [5,2,3], future = [8,5,6], hierarchy = [[1,2],[2,3]], budget = 7
           输出: 12
           解释: 1 → 2 → 3
                - 员工 1 以价格 5 购买股票，获得利润 8 - 5 = 3
                - 员工 2 可获得折扣价 floor(2 / 2) = 1，获得利润 5 - 1 = 4
                - 员工 3 可获得折扣价 floor(3 / 2) = 1，获得利润 6 - 1 = 5
                - 总成本为 5 + 1 + 1 = 7 <= budget，因此最大总利润为 3 + 4 + 5 = 12

        解: 使用 `树 DP`
            - 一棵以 `1` 为根的有向树
            - 每个节点只有一个父节点(除了 1)

            对于每个员工 i :
              - 原价买: 成本 present[i]
              - 折扣买(前提：直属上司买了): 成本 floor(present[i] / 2)
              - 收益: future[i] - cost
              - 只能买一次
              - 总成本 ≤ budget
             关键限制: 下属是否能打折，完全取决于父节点是否购买
          典型的: 树结构 + 状态依赖 + 背包预算

          DP 核心:
             对于员工 i:
             - 父节点没买: dp[i][0]
               dp[i][0][k] = 在父节点【没买】的前提下, 在 i 的子树中花费 k 的最大利润
             - 父节点买了: dp[i][1]
               dp[i][1][k] = 在父节点【已买】的前提下, 在 i 的子树中花费 k 的最大利润
             k 是花费(0 ~ budget)

        判断 DP 维数, 使用黄金法则:
         present[i]: 表示第 i 位员工今天可以购买股票的 `当前价格`
         future[i]: 表示第 i 位员工明天可以卖出股票的 `预期价格`

         1. 为了避免重复计算，我在递归/转移时，必须记住哪些信息?
            - 当前处理的是哪位员工(i)
            - 父节点有没有买股票
            - 已经花了多少钱
            - 已经赚了多少钱
            - 子节点有哪些
            - CEO 是谁

         2. 对每个变量问一句：不记录它，后续决策会不会出错？
            - 变量 A: 当前员工 i
              不记录 i 会发生什么？
              - 不知道 present[i]、future[i]
              - 不知道有哪些子员工
              - 子树完全不同
              👉递归结果不唯一
              ✅ 必须记录
              ➡️ 第 1 个维度：u

             - 变量 B: 父节点是否购买(parentBought)
               不记录它会发生什么？
               - 无法判断当前员工是否可以半价
               - 成本不确定
               - 子节点是否能打折也不确定
               👉后续决策直接错误
               ✅ 必须记录
               ➡️ 第 2 个维度：parentBought(0/1)

             - 变量 C: 已经赚了多少钱(profit)
               不记录会出错吗？
               因为:
                  - DP 的返回值本身就是「最大利润」
                  - 利润是 `结果`, 不是 `条件`
               👉 这是“要算的东西”，不是“状态”
               ❌ 不记录

             - 变量 D: 已经花了多少钱(cost)(成本)
               不记录会出错吗？
               - 无法判断是否超 budget
               - 无法在不同花费方案之间比较优劣
               👉 背包约束丢失
               ✅ 必须记录
               ➡️ 第 3 个维度：cost

              - 变量 E: 子节点有哪些
                这能从 i 推出来，不是独立变量
                ❌ 不记录

              - 变量 F: CEO 是谁
                CEO 固定为 1，是常量
                ❌ 不记录

            3. 能合并吗？
               现在得到 3 个 `必须记录` 的变量, 并逐个检查能否合并
               - i
               - parentBought
               - cost

               - i 和 parentBought 能合并吗?
                 - 同一个 i
                 - parentBought = 0/1 时，成本和转移完全不同
                 ❌ 不能合并

               - i 和 cost 能合并吗？
                 - cost 是连续区间
                 - i 是离散节点
                 ❌ 不能合并

               - parentBought 和 cost 能合并吗？
                 - parentBought 是逻辑状态
                 - cost 是资源消耗
                 ❌ 不能合并

              推导出结果:
                 DP 必须包含 3 个独立维度: `dp[i][parentBought][cost]`, 即: `dp[i][p][c]`

            dp[i][p][c]:
             - `dp` 的值是 `利润`
             - `c` 是 `成本(预算)`
             - `p` 只影响 `i 是否能打折`

            边界条件:
              叶子节点(i 没有下属)
               - 不买 i
                 - 成本 = 0
                 - 利润 = 0
               dp[i][0][0] = 0
               dp[i][1][0] = 0

               - 买 i
                 如果 p = 0(父节点没买)
                 - 成本 = present[i]
                 - 利润 = future[i] - present[i]
                 dp[i][0][present[i]] = future[i] - present[i]

                如果 p = 1(父节点买了，可打折)
                - 成本 = floor(present[i] / 2)
                - 利润 = future[i] - floor(present[i] / 2)
                dp[i][1][floor(present[i]/2)] = future[i] - floor(present[i]/2)

              状态转移方程:
               假设:
                  - i 有子节点
                  - 子节点已经算好了 dp
                  计算: dp[i][p][*]

               1. 先考虑「i 自己买 or 不买」
                  这是 `树 DP 的第一层决策`

                 情况 1: i 不买
                   - i 不花钱
                   - i 不给子节点提供折扣
                   子节点: dp[child][0][*]
                   初始状态: cur[c = 0] = 0
                   合并所有子节点(树形背包): cur = ⊕ dp[child][0]
                   合并完后: dp[i][p][c] = cur[c]
                   注意: 这里 与 p 无关，因为 i 没买

                 情况 2: i 买
                    先确定 i 的成本 & 利润
                      - 如果 p = 0
                        - cost_i = present[i]
                        - gain_i = future[i] - present[i]
                      - 如果 p = 1
                        - cost_i = floor(present[i] / 2)
                        - gain_i = future[i] - floor(present[i] / 2)
                    初始状态: cur[cost_i] = gain_i
                    合并所有子节点: 因为 i 买了 ⇒ 子节点可打折: cur = ⊕ dp[child][1]
                    更新 dp: dp[i][p][c] = max(dp[i][p][c], cur[c])

                    总结成一句转移逻辑:
                     对于每个 `i`、`p`:
                     dp[i][p] =
                          max(
                              「i 不买 + ⊕ dp[child][0]」,
                              「i 买   + ⊕ dp[child][1]」
                          )
                    其中「买」的成本取决于 p

                CEO 没有父节点，视为: p = 0, max(dp[1][0][c])  , c ≤ budget

       代码思路:
           1. 把 hierarchy 建成树
           2. DFS 后序遍历(子节点先算)
              - 折扣规则是 `从父 → 子` 传播的，但 DP 计算顺序必须是 `从子 → 父`（后序遍历）
           3. 在 dfs(i) 里算 dp[i][0][*] 和 dp[i][1][*]
           4. 根节点取 max(dp[1][0][c])

       为什么 `DFS 后序遍历` ?
         1. 规则
            `如果直属上司买了 → 员工可以半价`，但 `DP 不是在模拟过程，而是在算最优解的函数值`

         2. `规则方向` vs `计算依赖方向`
            - 规则方向(业务逻辑)
              `父是否购买  →  决定子是否打折`
              ✅规则是自顶向下的

            - 计算依赖方向(DP)
              计算 dp[i] 时，需要不需要知道 dp[子节点] ? → ✅ 必须需要
              - dp[i] = 自己的选择(买/不买), 所有子节点子树的最优结果
              👉 `父节点的 DP 值`，依赖 `子节点的 DP 值`
              ✅计算依赖是自底向上的

          3. 为什么 `先算父` 在 DP 里会直接失败?
             假设你要算 dp[i]
             dp[i] =
                  max(
                      不买 i + sum(dp[child][0]),
                      买 i   + sum(dp[child][1])
                  )
              👉 dp[child][0/1] 必须已经存在
              如果先算父节点:
              - 子节点 dp 还没算
              - dp[i] 无法计算
              ❌ 逻辑上就断了

           4. 那 `父节点是否购买` 是怎么传下去的？
              父是否购买 ≠ 先算父节点
              这里的 DP: dp[i][p] = 假设 `父节点状态是 p` 时，`i 子树` 的 `最优解`
              注意关键词: `假设`
              👉 DP 并不是说 `父已经真的买了`, 而是 `在父买 / 不买这两种前提下，i 子树最优能做到多少?`
              所以:
               - 父节点的 `买/不买`
               - `被编码成了状态 p`
               - 而不是通过遍历顺序传递

            5. 极其关键的对比
               ❌ 错误理解(时间顺序)
                  `先算父 → 再算子 → 再回头修正父`, 这叫 `模拟过程`，`不是 DP`

               ✅ 正确理解(函数依赖)
                  `dp[i] 是一个函数, 它的值 = 子节点 dp 的组合结果`
                  👉 所以:
                  ```markdown
                  子节点 dp 先算好
                        ↓
                  父节点 dp 才能算
                  ```
                  这就是 `后序遍历`

              6. 用一句概括
                 `DP 的遍历顺序，永远跟“状态依赖关系”走，而不是跟“题目描述的因果关系”走`

              7. 总结
                 - 折扣规则: 父 → 子(业务)
                 - DP 依赖: 子 → 父(计算)
                 - 解决方式:
                   - 👉 `用状态 p 表示父节点是否购买`
                   - 👉 `用后序遍历计算 dp`

         步骤一: 把 hierarchy 建成树
         1. hierarchy 含义
           [boss, employee]
           - boss: 直属上司
           - employee: boss 的直接下属
          ```
          hierarchy = [[1, 2], [1, 3], [2, 4]]
          [1, 2] → 1 是 2 的上司
          [1, 3] → 1 是 3 的上司
          [2, 4] → 2 是 4 的上司

          假设: n = 4
          children = [
            [], // 0 -> 员工 1
            [], // 1 -> 员工 2
            [], // 2 -> 员工 3
            [], // 3 -> 员工 4
          ]

          1. 处理 [1, 2]
             boss = 1 - 1 = 0
             emp = 2 - 1 = 0
             children[boss].push(emp) = children[0].push(1)
             children = [
                [1],
                [],
                [],
                [],
             ]

             1
             └── 2

          2. 处理 [1, 3]
             boss = 1 - 1 = 0
             emp = 3 - 1 = 2
             children[boss].push(emp) = children[0].push(2)
             children = [
                [1, 2],
                [],
                [],
                [],
             ]

            1
            ├── 2
            └── 3

            3. 处理 [2, 4]
             boss = 2 - 1 = 1
             emp = 4 - 1 = 3
             children[boss].push(emp) = children[1].push(3)
             children = [
                [1, 2],
                [3],
                [],
                [],
             ]

            1
            ├── 2
            │   └── 4
            └── 3

            最终:
              children[0] = [1, 2]   → 1 的下属是 2, 3
              children[1] = [3]      → 2 的下属是 4
              children[2] = []       → 3 没下属
              children[3] = []       → 4 没下属

            员工 1
            ├── 员工 2
            │   └── 员工 4
            └── 员工 3
          ```

          步骤二: DFS 后序遍历(子节点先算)
          后序遍历: 4 → 2 → 3 → 1
          即:
            dfs(1)
              ├─ dfs(2)
              │   └─ dfs(4)
              └─ dfs(3)


          推导条件: child_p = if c1==0 {0} else {1}
          1. 树结构:
            1
            ├── 2
            │   ├── 4
            │   └── 5
            └── 3
            - 根节点 1 → 子节点 2,3
            - 节点 2 → 子节点 4,5
            - 节点 3 → 没有子节点

            假设:
            节点             价格(present)             预期价格(future)
            1                4                        10
            2                3                        7
            3                5                        11
            4                2                        4
            5                1                        3

          2. 分析父 → 子折扣逻辑
             折扣规则: 如果父节点买了，子节点可以 `半价购买`

             1️⃣ 节点 2 合并第一个子节点 4
              - 假设 `节点 2 不买(花费0)`:
                - c1 = 0 → 父没买
                - 子节点 4 打折吗？不打折 ❌
                - Hack 判断：c1 > 0 ? → 0 → 正确

              - 假设 `节点 2 买了(花费 3)`
                - c1 = 3 → 父买了
                - 子节点 4 打折吗？可以打折 ✅
                - Hack 判断：c1 > 0 ? → 1 → 正确
              第一个子节点合并时，hack 和实际逻辑完全一致

             2️⃣ 节点 2 合并第二个子节点 5
              - 假设 `节点 2 买了(花费 3)`, 同时 `节点 4 也买了(花费 2)`
                - 累积 c1 = 3 + 2 = 5
                - 合并子节点 5
                  - Hack 判断: c1 > 0 ? → 1 → 假设父节点买了
                  - 实际情况: 父节点确实买了 → ✅ 正确

              - 假设 `节点 2 没买(花费 0)`, 同时 `节点 4 买了(花费 2)`
                - 累积 c1 = 0 + 2 = 2
                - 合并子节点 5
                  - Hack 判断: c1 > 0 ? → 1 → 假设父节点买了
                  - 实际情况: 父节点 2 没买，子节点 5 不该打折 ❌
              这就清楚了: 当有多个子节点时，c1>0 不再唯一来源于父节点购买，hack 会误判折扣

              3️⃣ 节点 1 合并子节点 2 和 3
               - 节点 1 花费 = 4
               - 节点 2 花费累积 = 5
               - 节点 2 花费 = 5
               - Hack 判断 c1 > 0 → 1
               - 实际情况: 折扣传递只看 父节点 1 是否购买，而不是兄弟节点花费
               - 如果把兄弟节点花费也算进去，hack 会错误地认为父节点买了 → 折扣传递错误

             总结:
              阶段              c1 来源                         Hack 判断           正确性
              第一个子节点       父节点是否购买                    c1>0 → 正确          ✅
              第二个子节点       父节点是否购买+ 第一个子节点花费     c1>0 → 可能错误      ❌
              多兄弟节点累积     父节点 + 之前所有子节点             c1>0 → 不可靠       ❌


*/

/// 返回最大利润
/// n: 员工数量
/// present: 当前价格
/// future: 明天预期价格
/// hierarchy: 上下级关系 [boss, employee]
/// budget: 总投资预算
pub fn max_profit(
    n: i32,
    present: Vec<i32>,
    future: Vec<i32>,
    hierarchy: Vec<Vec<i32>>,
    budget: i32,
) -> i32 {
    if budget <= 0 || n <= 0 {
        return 0;
    }

    let n = n as usize;
    let budget = budget as usize;

    // children[i] = i 的所有直属下属, children 是树的邻接表表示
    let mut children = vec![Vec::new(); n];

    // 1. 把 hierarchy 建成树
    // 把 `父 → 子` 的关系，存进 `children[父]` 里
    /*
         员工 1
           ├── 员工 2
           │   └── 员工 4
           └── 员工 3

           children = [
               [1, 2], -- 员工 2、员工 3
               [3],    -- 员工 4
               [],
               [],
            ]
    */
    for e in hierarchy {
        let boss = (e[0] - 1) as usize; // 领导
        let emp = (e[1] - 1) as usize; // 雇员
        children[boss].push(emp);
    }

    // 2. DFS 后序遍历(子节点先算) → (dp0, dp1, subtree_size)
    // 先算所有子节点能贡献的利润（不管 u 买不买）
    // 再决定 u 自己买 or 不买
    // 最后把 `u 的选择” + “子树的最优结果` 合在一起
    fn dfs(
        u: usize,
        children: &Vec<Vec<usize>>,
        present: &Vec<i32>,
        future: &Vec<i32>,
        budget: usize,
    ) -> (Vec<i32>, Vec<i32>, usize) {
        let cost = present[u] as usize; // 全价购买
        let d_cost = (present[u] / 2) as usize; // 半价购买

        // 考虑 u 自己 + 子节点
        let mut dp0 = vec![0; budget + 1]; // 父节点没买, 在 u 这棵子树中，用 i 的预算，最大收益
        let mut dp1 = vec![0; budget + 1]; // 父节点买了, 在 u 这棵子树中，用 i 的预算，最大收益

        /*
         子节点（很多个）
             ↓  树形背包合并
         sub_profit（只看下属）
             ↓  决定 u 买 or 不买
         dp（u + 子树的最终结果）
        */
        // 只考虑子节点
        let mut sub_profit0 = vec![0; budget + 1]; // 不买 u，且 u 的父节点“没买”, 在 u 的所有子节点中，用 i 的预算，能得到的最大收益
        let mut sub_profit1 = vec![0; budget + 1]; // 不买 u，但 u 的父节点“买了”(优惠可向下传播), 在 u 的所有子节点中，用 i 的预算，能得到的最大收益

        // 当前子树最大花费(优化背包循环)
        let mut u_size = cost; // `这个子树最多可能花多少钱` 的上界, 限制背包循环的范围，避免无意义遍历

        for &v in &children[u] {
            let (child_dp0, child_dp1, v_size) = dfs(v, children, present, future, budget);
            u_size += v_size;

            // 把 `一个子节点 v 的所有可能花费方案`, 合并进`当前已经合并好的子树方案` 里
            // 使用 rev(), 这是 0/1 背包的铁律: 防止一个子节点被重复使用, 每个子节点 `只能算一次`
            // 每次合并子节点时，只允许: 用合并之前的状态，更新合并之后的状态
            // 🔒树形背包合并子树 = 01 背包 = 必须倒序
            for i in (0..=budget).rev() {
                // 分给 `当前子节点 v` 的预算
                // 👉sub: 这个子节点的整棵子树，花多少钱
                /*
                 两个硬限制:
                 1. 子树本身最多能花这么多钱
                    - 子节点 v 的子树
                    - 不可能花超过 v_size
                    👉所以: sub ≤ v_size

                  2. 当前总预算是 i
                     - 总花费 = i
                     - 子树花了 sub
                     - 剩下的是 i - sub
                     👉所以: sub ≤ i
                  所以: 范围为: sub ∈ [0, min(v_size, i)]
                */
                // 两层循环: 把 `一个子节点的整棵子树`, 当成一个 01 背包物品, 按不同花费 sub, 合并进当前节点的背包里
                for sub in 0..=v_size.min(i) {
                    if i >= sub {
                        /*
                          sub_profit0[i - sub]:
                          👉在不考虑当前子节点之前, 已经花了 i - sub 的钱, 能拿到的最大收益

                          child_dp0[sub]:
                          👉当前这个子节点 v, 如果它的父节点没买, 在子树里花 sub 的钱, 能得到的最大收益

                          已有子节点的最优解(i - sub) + 当前子节点 v 的贡献(sub) 👉 正好凑成总花费 = i
                          经典背包问题: dp[i] = max(dp[i], dp[i - w] + v)
                          - w = sub
                          - v = child_dp[sub]
                          - 物品 = “子节点整棵子树”
                        */
                        // 在 u 不买 的前提下, 如果「之前合并的子节点花了 i - sub, 当前子节点 v 花了 sub」, 那么「总花费 = i, 总利润 = 之前子节点的利润 + v 子树的利润」
                        sub_profit0[i] = sub_profit0[i].max(sub_profit0[i - sub] + child_dp0[sub]);
                        sub_profit1[i] = sub_profit1[i].max(sub_profit1[i - sub] + child_dp1[sub]);
                    }
                }
            }
        }

        // ----- 合并自身节点 -----
        for i in 0..=budget {
            dp0[i] = sub_profit0[i]; // 父没买，不买自己
            dp1[i] = sub_profit0[i]; // 父买了，不买自己

            // 买自己，父买了可打折, 买 u >= d_cost(成本价, 打折)
            if i >= d_cost {
                // i - d_cost: 总预算: i, 买 u 花了 d_cost, 剩下: i - d_cost, 剩下的只能给子节点
                // sub_profit1[i - d_cost]: u 被买了, 子节点可以享受折扣
                // future[u] - d_cost: 净收益
                dp1[i] = dp1[i].max(sub_profit1[i - d_cost] + future[u] - d_cost as i32);
            }

            // 买自己，父没买原价, 不买 u >= cost(成本价, 不打折)
            // i - cost: 总预算: i, 买 u 花了 cost, 剩下: i - cost, 剩下的只能给子节点
            // ub_profit1:[i - cost]: u 被买了, 子节点不能享受折扣
            // future[u] - cost: 净收益
            if i >= cost {
                dp0[i] = dp0[i].max(sub_profit1[i - cost] + future[u] - cost as i32);
            }
        }

        (dp0, dp1, u_size)
    }

    // 3. 从 CEO（员工 1，对应 index 0）开始 DFS
    let (dp0, _, _) = dfs(0, &children, &present, &future, budget);

    // 4. 根节点最大利润
    // 根节点(CEO)没有父节点，等价于「父节点没买」，所以只能取 `dp0`
    // dp0[budget] = 在不超过 budget 的情况下的最大利润
    dp0[budget]
}
