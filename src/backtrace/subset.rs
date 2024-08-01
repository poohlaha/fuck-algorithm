//! 回溯算法 --- 子集

/// 子集 - 元素无重不可复选
/// 输入一个无重复元素的数组 nums，其中每个元素最多使用一次，请你返回 nums 的所有子集。
/// 使用 start 参数控制树枝的生长避免产生重复的子集，用 track 记录根节点到每个节点的路径的值，同时在前序位置把每个节点的路径值收集起来，完成回溯树的遍历就收集了所有子集
pub(crate) fn subsets(nums: Vec<u32>) -> Vec<Vec<u32>> {
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
