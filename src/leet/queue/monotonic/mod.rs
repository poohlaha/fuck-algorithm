use std::collections::VecDeque;

pub struct Monotonic;

impl Monotonic {
    pub fn max_sliding_window2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return Vec::new();
        }

        if k <= 0 {
            return Vec::new();
        }

        if k == 1 {
            return nums;
        }

        // 创建 `deque` 空队列, 用于 `递减` 存放 `元素下标`
        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut results: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            // 检查队头是否过期
            if let Some(&front) = deque.front() {
                if front + (k as usize) < i {
                    // 过期, 弹出队头
                    deque.pop_front();
                }
            }

            // 从队尾弹出所有小于 nums[i] 的值
            while let Some(&back) = deque.back() {
                if nums[i] > nums[back] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            // 加入新的元素下标
            deque.push_back(i);

            if i >= (k as usize) - 1 {
                if let Some(&front) = deque.front() {
                    results.push(nums[front]);
                }
            }
        }

        results
    }

    /**
      239. 滑动窗口最大值
      题目: 给你一个整数数组 `nums`，有一个大小为 `k` 的滑动窗口从数组的最左侧移动到数组的最右侧。
           你只可以看到在滑动窗口内的 `k` 个数字。
           滑动窗口每次只向右移动一位

      解: 使用 `单调队列(递减)`, 队头为每个窗口最大值
    */
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return Vec::new();
        }

        if k <= 0 {
            return Vec::new();
        }

        if k == 1 {
            return nums;
        }

        // 创建 `left`、`right` 指针, `left = 0`, `right = k - 1`(k 为窗口大小)
        let mut left: usize = 0;
        let mut right: usize = (k - 1) as usize;

        // 创建 `deque` 空队列, 用于 `递减` 存放 `元素下标`
        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut results: Vec<i32> = Vec::new();

        while right < nums.len() {
            if left == 0 && deque.len() == 0 {
                // 首次插入
                for i in left..=right {
                    // 队列为空, 插入元素
                    if deque.len() == 0 {
                        deque.push_back(i);
                        continue;
                    }

                    while let Some(&back) = deque.back() {
                        if nums[i] > nums[back] {
                            deque.pop_back();
                        } else {
                            break;
                        }
                    }

                    // 插入新元素下标
                    deque.push_back(i);
                }

                let front = deque.front();
                if let Some(&front) = front {
                    results.push(nums[front]);
                }

                left += 1;
                right += 1;
                continue;
            }

            // 检查队头是否过期
            let dep = deque.front();
            if let Some(&dep) = dep {
                if left > dep {
                    // 过期, 弹出队头
                    deque.pop_front();
                }
            }

            while let Some(&back) = deque.back() {
                if nums[right] > nums[back] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            // 插入新元素下标
            deque.push_back(right);
            let dep = deque.front();
            if let Some(&dep) = dep {
                results.push(nums[dep]);
            }

            left += 1;
            right += 1;
        }

        results
    }

    /**
      1438. 绝对差不超过限制的最长连续子数组
      题目: 给你一个整数数组 nums ，和一个表示限制的整数 limit，请你返回最长连续子数组的长度，该子数组中的任意两个元素之间的绝对差必须小于或者等于 limit
           如果不存在满足条件的子数组，则返回 0

      解: 使用 `单调队列(递减)` 和 `单调队列(递增), 两队头相减 <= limit
    */
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut max_deque: VecDeque<usize> = VecDeque::new(); // 递减
        let mut min_deque: VecDeque<usize> = VecDeque::new(); // 递增

        let mut max_len = 0;
        let mut left = 0;
        for i in 0..nums.len() {
            // 去除 max_deque 中不满足要求的元素下标
            while let Some(&back) = max_deque.back() {
                if nums[i] > nums[back] {
                    max_deque.pop_back();
                } else {
                    break;
                }
            }

            // 去除 min_deque 中不满足要求的元素下标
            while let Some(&back) = min_deque.back() {
                if nums[i] < nums[back] {
                    min_deque.pop_back();
                } else {
                    break;
                }
            }

            // 加入新元素
            max_deque.push_back(i);
            min_deque.push_back(i);

            // 检查窗口是否符合条件
            while nums[*max_deque.front().unwrap()] - nums[*min_deque.front().unwrap()] > limit {
                if *max_deque.front().unwrap() == left {
                    max_deque.pop_front();
                }
                if *min_deque.front().unwrap() == left {
                    min_deque.pop_front();
                }
                left += 1;
            }

            max_len = max_len.max(i - left + 1);
        }

        max_len as i32
    }
}
