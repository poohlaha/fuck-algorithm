//! 回溯算法 --- 子集

/// 子集 - 元素无重不可复选
/// 输入一个无重复元素的数组 nums，其中每个元素最多使用一次，请你返回 nums 的所有子集。
/// 使用 start 参数控制树枝的生长避免产生重复的子集，用 track 记录根节点到每个节点的路径的值，同时在前序位置把每个节点的路径值收集起来，完成回溯树的遍历就收集了所有子集
pub(crate) fn subsets(nums: Vec<u32>) -> Vec<Vec<u32>> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut results: Vec<Vec<u32>> = Vec::new();
    let mut track: Vec<u32> = Vec::new();

    fn backtrace(results: &mut Vec<Vec<u32>>, track: &mut Vec<u32>, nums: &Vec<u32>, start: usize) {
        // 前序遍历位置，每个节 点的值都是一个子集
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

/// 组合 - 元素无重不可复选
/// 组合和子集是一样的：大小为 k 的组合就是大小为 k 的子集。
/// 给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。
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

/// 子集 - 元素可重不可复选
/// 给你一个整数数组 nums，其中可能包含重复元素，请你返回该数组所有可能的子集
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

/// 输入 nums 和一个目标和 target，从 nums 中找出中所有和为 target 的组合。
/// candidates 可能存在重复元素，且其中的每个数字最多只能使用一次。
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

/// 排列 - 元素可重不可复选
/// 输入一个可包含重复数字的序列 nums，请你写一个算法，返回所有可能的全排列
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

/// 子集/组合 - 元素无重可复选
/// 输入数组无重复元素，但每个元素可以被无限次使用
/// 想让每个元素被重复使用，我只要把 i + 1 改成 i 即可, 这相当于给之前的回溯树添加了一条树枝，在遍历这棵树的过程中，一个元素可以被无限次使用
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

/// 排列 - 元素无重可复选
/// nums 数组中的元素无重复且可复选的情况下，会有哪些排列
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
