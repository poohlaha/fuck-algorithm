/**!
   回溯算法 --- 子集/组合/排列
   1. 元素无重不可复选，即 nums 中的元素都是唯一的，每个元素最多只能被使用一次，这也是最基本的形式。
      以组合为例，如果输入 nums = [2,3,6,7]，和为 7 的组合应该只有 [7]。

   2. 元素可重不可复选，即 nums 中的元素可以存在重复，每个元素最多只能被使用一次。
      以组合为例，如果输入 nums = [2,5,2,1,2]，和为 7 的组合应该有两种 [2,2,2,1] 和 [5,2]

   3. 元素无重可复选，即 nums 中的元素都是唯一的，每个元素可以被使用若干次
      以组合为例，如果输入 nums = [2,3,6,7]，和为 7 的组合应该有两种 [2,2,3] 和 [7]。

   组合问题和子集问题其实是等价的；至于之前说的三种变化形式，无非是在下面两棵树上剪掉或者增加一些树枝罢了。

   组合/子集树
                []
           /    |    \
         1      2     3
       /   \    |
      2     3   3
      |
      3

    排列树
                  []
           /       |        \
         1         2         3
       /   \     /   \     /   \
      2     3   1     3   1     2
      |     |   |     |   |     |
      3     2   3     1   2     1
*/

/**
 球盒模型
 排列: P(n, k) = n! / (n - k)!
 组合: C(n, k) = n! / k!(n - k)!

 排列:
 1. 站在 `盒子` 的视角，每个盒子必然要选择一个球
    第一个盒子可以选择 `n` 个球中的任意一个，然后你需要让剩下 `k - 1` 个盒子在 `n - 1` 个球中选择（这就是子问题 P(n - 1, k - 1))
 2. 可以站在 `球` 的视角，因为并不是每个球都会被装进盒子，所以球的视角分两种情况
    - 第一个球可以不装进任何一个盒子，这样的话你就需要将剩下 `n - 1` 个球放入 `k` 个盒子。
    - 第一个球可以装进 `k` 个盒子中的任意一个，这样的话你就需要将剩下 `n - 1` 个球放入 `k - 1` 个盒子。

 组合:
 不需要对盒进行编号，可以认为只有一个盒，其容量是 k。
 1. 站在 `盒子` 的视角，盒子必然要装 k 个球。
    那么第一个球可以选择 n 个球中的任意一个，然后盒子剩余 k - 1 个位置，你需要在剩下的 n - 1 个球中选择（这就是子问题 C(n - 1, k - 1)）。
    直接 nC(n - 1, k - 1) 是有重复的，正确的答案应该是 nC(n - 1, k - 1) / k
 2. 站在 `球` 的视角，因为并不是每个球都会被装进盒子，所以球的视角分两种情况
    - 第一个球可以不装进盒子，这样的话你就需要将剩下 n - 1 个球放入 k 个盒子。
    - 第一个球可以装进盒子，这样的话你就需要将剩下 n - 1 个球放入 k - 1 个盒子。
*/

/**
   子集 - 元素无重不可复选, 以 `盒` 视角
   力扣: https://leetcode.cn/problems/subsets/description/
   题目: 输入一个无重复元素的数组 nums，其中每个元素最多使用一次，请你返回 nums 的所有子集。
   答: 输入 nums = [1,2,3]，算法应该返回如下子集: [ [],[1],[2],[3],[1,2],[1,3],[2,3],[1,2,3] ]

   保证元素之间的相对顺序不变来防止出现重复的子集:
                  []
         1/       2|      3\
        [1]       [2]     [3]
     2/   3\       3|
    [1,2]  [1,3]  [2,3]        --- 这一层为大小为 2 的子集
      |
    [1,2,3]

    这颗树的特性:
    如果把根节点作为第 0 层，将每个节点和根节点之间树枝上的元素作为该节点的值，那么第 n 层的所有节点就是大小为 n 的所有子集。
    如果想计算所有子集，那只要遍历这棵多叉树，把所有节点的值收集起来不就行了

    解:
    使用 start 参数控制树枝的生长避免产生重复的子集，用 track 记录根节点到每个节点的路径的值，同时在前序位置把每个节点的路径值收集起来，完成回溯树的遍历就收集了所有子集。
*/
pub(crate) fn subsets(nums: Vec<u32>) -> Vec<Vec<u32>> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut results: Vec<Vec<u32>> = Vec::new();
    let mut track: Vec<u32> = Vec::new();

    fn backtrace(results: &mut Vec<Vec<u32>>, track: &mut Vec<u32>, nums: &Vec<u32>, start: usize) {
        // 前序遍历位置，每个节点的值都是一个子集
        results.push(track.clone());

        for i in start..nums.len() {
            track.push(nums[i]);
            backtrace(results, track, nums, i + 1);
            track.pop();
        }
    }

    backtrace(&mut results, &mut track, &nums, 0);
    return results;
}

/**
   子集 - 元素无重不可复选, 以 `球` 视角
   站在 `球` 的视角，因为并不是每个球都会被装进盒子，所以球的视角分两种情况
    - 第一个球可以不装进盒子，这样的话你就需要将剩下 n - 1 个球放入 k 个盒子。
    - 第一个球可以装进盒子，这样的话你就需要将剩下 n - 1 个球放入 k - 1 个盒子。
*/
pub(crate) fn subsets2(nums: Vec<u32>) -> Vec<Vec<u32>> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut results: Vec<Vec<u32>> = Vec::new();
    let mut track: Vec<u32> = Vec::new();

    fn backtrace(results: &mut Vec<Vec<u32>>, track: &mut Vec<u32>, nums: &Vec<u32>, start: usize) {
        if start == nums.len() {
            results.push(track.clone());
            return;
        }

        // 做第一种选择，元素在子集中
        track.push(nums[start]);
        backtrace(results, track, nums, start + 1);
        track.pop(); // 撤销选择

        // 做第二种选择，元素不在子集中
        backtrace(results, track, nums, start + 1);
    }

    backtrace(&mut results, &mut track, &nums, 0);
    return results;
}

/**
   组合 - 元素无重不可复选
   力扣: https://leetcode.cn/problems/combinations/description/
   题目: 给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。(输入一个数组 nums = [1,2..,n] 和一个正整数 k，请你生成所有大小为 k 的子集)
   答: 输入  = 3, k = 2，算法应该返回如下组合: [ [1,2],[1,3],[2,3] ]
   组合和子集是一样的：大小为 k 的组合就是大小为 k 的子集。

                   []
          1/       2|      3\
         [1]       [2]     [3]
      2/   3\       3|
     [1,2]  [1,3]  [2,3]        --- 这一层为大小为 2 的组合
       |
     [1,2,3]

    解: 只需要把第 2 层（根节点视为第 0 层）的节点收集起来，就是大小为 2 的所有组合
*/
pub(crate) fn combine(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut results: Vec<Vec<usize>> = Vec::new();
    let mut track: Vec<usize> = Vec::new();

    fn backtrace(
        results: &mut Vec<Vec<usize>>,
        track: &mut Vec<usize>,
        start: usize,
        n: usize,
        k: usize,
    ) {
        if k == track.len() {
            // 遍历到了第 k 层，收集当前节点的值
            results.push(track.clone());
            return;
        }

        for i in start..=n {
            track.push(i);
            // 通过 start 参数控制树枝的遍历，避免产生重复的子集
            backtrace(results, track, i + 1, n, k);
            // 撤销选择
            track.pop();
        }
    }

    backtrace(&mut results, &mut track, 1, n, k);
    return results;
}

/**
  子集 - 元素可重不可复选
  力扣: https://leetcode.cn/problems/subsets-ii/description/
  题目: 给你一个整数数组 nums ，其中可能包含重复元素，请你返回该数组所有可能的子集（幂集）。
  答: 比如输入 nums = [1,2,2]，你应该输出: [ [],[1],[2],[1,2],[2,2],[1,2,2] ]
  以 nums = [1,2,2] 为例，为了区别两个 2 是不同元素，后面我们写作 nums = [1,2,2']

                    []
        1/          2|      2'(x)\
       []           [2]         [2']
    2/   2'(x)\    2'|
   [1,2]  [1,2]     [  ]
     |
    [ ]
   其中 x 表示需要剪枝。

   剪枝条件:
   if (i > start && nums[i] === nums[i - 1]) {
      continue;
   }
*/
pub(crate) fn subsets_with_dup(nums: Vec<u32>) -> Vec<Vec<u32>> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut new_nums = nums.clone();
    // 升序排列，方便剪枝
    new_nums.sort_by(|a, b| a.cmp(b));

    let mut results: Vec<Vec<u32>> = Vec::new();
    let mut track: Vec<u32> = Vec::new();

    fn backtrace(results: &mut Vec<Vec<u32>>, track: &mut Vec<u32>, nums: &Vec<u32>, start: usize) {
        // 前序遍历位置，每个节 点的值都是一个子集
        results.push(track.clone());

        for i in start..nums.len() {
            // 剪枝逻辑，值相同的相邻树枝，只遍历第一条
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }

            track.push(nums[i]);
            backtrace(results, track, nums, i + 1);
            track.pop();
        }
    }

    backtrace(&mut results, &mut track, &new_nums, 0);
    return results;
}

/**
    组合 - 元素可重不可复选
    力扣: https://leetcode.cn/problems/combination-sum-ii/description/
    题目: 给定一个候选人编号的集合 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。candidates 中的每个数字在每个组合中只能使用 一次 。注意：解集不能包含重复的组合。
    同 【 子集 - 元素可重不可复选】

    剪枝条件:
     if (i > start && nums[i] === nums[i - 1]) {
        continue;
     }

    // 达到目标和，找到符合条件的组合，记录结果
     if sum == target {
         results.push(track.clone());
         return;
     }

     // 先剪枝，超过目标和，直接结束
     if sum.clone() > target {
         return;
     }
*/
pub(crate) fn combination_sum(nums: Vec<u32>, target: u32) -> Vec<Vec<u32>> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut new_nums = nums.clone();
    // 升序排列，方便剪枝
    new_nums.sort_by(|a, b| a.cmp(b));

    let mut results: Vec<Vec<u32>> = Vec::new();
    let mut track: Vec<u32> = Vec::new();
    let sum: u32 = 0;

    fn backtrace(
        results: &mut Vec<Vec<u32>>,
        track: &mut Vec<u32>,
        nums: &Vec<u32>,
        mut sum: u32,
        start: usize,
        target: u32,
    ) {
        // 达到目标和，找到符合条件的组合，记录结果
        if sum == target {
            results.push(track.clone());
            return;
        }

        // 先剪枝，超过目标和，直接结束
        if sum.clone() > target {
            return;
        }

        for i in start..nums.len() {
            // 剪枝逻辑，值相同的相邻树枝，只遍历第一条
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }

            track.push(nums[i]);
            sum += nums[i];
            backtrace(results, track, nums, sum, i + 1, target);

            // 撤销选择
            track.pop();
            sum -= nums[i]
        }
    }

    backtrace(&mut results, &mut track, &new_nums, sum, 0, target);
    return results;
}

/**
    排列 - 元素可重不可复选
    力扣: https://leetcode.cn/problems/permutations-ii/description/
    题目: 给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。
    答: 输入 nums = [1,2,2]，函数返回：[ [1,2,2],[2,1,2],[2,2,1] ]
    新增了 !used[i - 1] 的逻辑判断。
    保证相同元素在排列中的相对位置保持不变。
    标准全排列算法之所以出现重复，是因为把相同元素形成的排列序列视为不同的序列，但实际上它们应该是相同的；而如果固定相同元素形成的序列顺序，当然就避免了重复。

    剪枝条件:
    // 新添加的剪枝逻辑，固定相同的元素在排列中的相对位置
    if (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
        // 如果前面的相邻相等元素没有用过，则跳过
        continue;
    }

    排列树:
                       []
          2/          2'|        2''\
     2'/   2''\     2/  2''|    2/  2'|
      |     |       |    |      |    |
    2''     2'     2''   2     2'    2

    使用 !used[i - 1] 剪枝:
                          []
         2/             2'(x)|        2''(x)\
     2'/   2''(x)\    2/   2''|      2/    2'|
      |       |       |     |        |      |
    2''       2'     2''    2        2'     2

    其中 x 表示需要剪枝。

*/
pub(crate) fn permute_unique(nums: Vec<u32>) -> Vec<Vec<u32>> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut new_nums = nums.clone();
    // 升序排列，方便剪枝
    new_nums.sort_by(|a, b| a.cmp(b));

    let mut results: Vec<Vec<u32>> = Vec::new();
    let mut track: Vec<u32> = Vec::new();

    let max = new_nums.len() as usize;
    let mut used = vec![false; max];

    fn backtrace(
        results: &mut Vec<Vec<u32>>,
        track: &mut Vec<u32>,
        nums: &Vec<u32>,
        used: &mut Vec<bool>,
    ) {
        // 前序遍历位置，每个节 点的值都是一个子集
        if track.len() == nums.len() {
            results.push(track.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] {
                continue;
            }

            // 新添加的剪枝逻辑，固定相同的元素在排列中的相对位置
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }

            track.push(nums[i]);
            used[i] = true;
            backtrace(results, track, nums, used);
            track.pop();
            used[i] = false;
        }
    }

    backtrace(&mut results, &mut track, &new_nums, &mut used);
    return results;
}

/**
   子集/组合 - 元素无重可复选
   力扣: https://leetcode.cn/problems/combination-sum/description/
   题目:
       给你一个 无重复元素 的整数数组 candidates 和一个目标整数 target ，找出 candidates 中可以使数字和为目标数 target 的 所有 不同组合 ，并以列表形式返回。你可以按 任意顺序 返回这些组合。
       candidates 中的 同一个 数字可以 无限制重复被选取 。如果至少一个数字的被选数量不同，则两种组合是不同的。
       对于给定的输入，保证和为 target 的不同组合数少于 150 个。
   答: 输入 candidates = [1,2,3], target = 3，算法应该返回：[ [1,1,1],[1,2],[3] ]

   想解决这种类型的问题，也得回到回溯树上，我们不妨先思考思考，标准的子集/组合问题是如何保证不重复使用元素的？
   答案在于 backtrack 递归时输入的参数 start, 这个 i 从 start 开始，那么下一层回溯树就是从 start + 1 开始，从而保证 nums[start] 这个元素不会被重复使用,
   那么反过来，如果我想让每个元素被重复使用，我只要把 i + 1 改成 i 即可。
   这相当于给之前的回溯树添加了一条树枝，在遍历这棵树的过程中，一个元素可以被无限次使用:

                              []
                  1/            2|       3\
                [ ]             [ ]       [ ]
             1/  2｜  3\      2/   3\     3｜
           [ ]   [ ]   [ ]   [ ]  [ ]     [ ]
              ...              ...        ...
   递归函数需要设置合适的 base case 以结束算法，即路径和大于 target 时就没必要再遍历下去了。
*/
pub(crate) fn combination_sum2(nums: Vec<u32>, target: u32) -> Vec<Vec<u32>> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut new_nums = nums.clone();
    // 升序排列，方便剪枝
    new_nums.sort_by(|a, b| a.cmp(b));

    let mut results: Vec<Vec<u32>> = Vec::new();
    let mut track: Vec<u32> = Vec::new();
    let sum: u32 = 0;

    fn backtrace(
        results: &mut Vec<Vec<u32>>,
        track: &mut Vec<u32>,
        nums: &Vec<u32>,
        mut sum: u32,
        start: usize,
        target: u32,
    ) {
        // 达到目标和，找到符合条件的组合，记录结果
        if sum == target {
            results.push(track.clone());
            return;
        }

        // 超过目标和，停止向下遍历
        if sum.clone() > target {
            return;
        }

        for i in start..nums.len() {
            // 剪枝逻辑，值相同的相邻树枝，只遍历第一条
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }

            track.push(nums[i]);
            sum += nums[i];
            backtrace(results, track, nums, sum, i, target);

            // 撤销选择
            track.pop();
            sum -= nums[i]
        }
    }

    backtrace(&mut results, &mut track, &new_nums, sum, 0, target);
    return results;
}

/**
   排列 - 元素无重可复选
   题目: nums 数组中的元素无重复且可复选的情况下，会有哪些排列？
   答: 输入 nums = [1,2,3]，那么这种条件下的全排列共有 3^3 = 27 种:
   [
      [1,1,1],[1,1,2],[1,1,3],[1,2,1],[1,2,2],[1,2,3],[1,3,1],[1,3,2],[1,3,3],
      [2,1,1],[2,1,2],[2,1,3],[2,2,1],[2,2,2],[2,2,3],[2,3,1],[2,3,2],[2,3,3],
      [3,1,1],[3,1,2],[3,1,3],[3,2,1],[3,2,2],[3,2,3],[3,3,1],[3,3,2],[3,3,3]
   ]

    标准的全排列算法利用 used 数组进行剪枝，避免重复使用同一个元素。如果允许重复使用元素的话，直接放飞自我，去除所有 used 数组的剪枝逻辑就行了。
*/
pub(crate) fn permute_repeat(nums: Vec<u32>) -> Vec<Vec<u32>> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut new_nums = nums.clone();
    // 升序排列，方便剪枝
    new_nums.sort_by(|a, b| a.cmp(b));

    let mut results: Vec<Vec<u32>> = Vec::new();
    let mut track: Vec<u32> = Vec::new();

    fn backtrace(results: &mut Vec<Vec<u32>>, track: &mut Vec<u32>, nums: &Vec<u32>) {
        // 前序遍历位置，每个节 点的值都是一个子集
        if track.len() == nums.len() {
            results.push(track.clone());
            return;
        }

        for i in 0..nums.len() {
            track.push(nums[i]);
            backtrace(results, track, nums);
            track.pop();
        }
    }

    backtrace(&mut results, &mut track, &new_nums);
    return results;
}
