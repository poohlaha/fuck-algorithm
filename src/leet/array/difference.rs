/*!
    370. 区间加法
    地址: https://leetcode.cn/problems/range-addition/description/
    问题: 假设你有一个长度为 `n` 的数组, 初始情况下所有的数字均为 `0`, 你将会被给出 `k` 个更新的操作
         其中, 每个操作会被表示一个三元组: `[startIndex, endIndex, inc]`, 你需要将子数组 `A[startIndex ... endIndex]` 包括(startIndex 和 endIndex) 增加 `inc`。
         请你返回 `k` 次操作后的数组

    解:
      使用差分数组:
      D[l] += x
      D[r + 1] -= x (r + 1 <= n)

   时间复杂度:
    - 单次区间修改: O(1)
    - 全部恢复成原数组: O(n)
*/

/*!
  1094. 拼车
  地址: https://leetcode.cn/problems/car-pooling/description/
  问题: 车上最初有 `capacity` 个空座位。车 `只能` 向一个方向行驶（也就是说，`不允许掉头或改变方向`）
       给定整数 `capacity` 和一个数组 `trips` ,  `trip[i] = [numPassengersi, fromi, toi] `表示第 i 次旅行有 `numPassengersi` 乘客，接他们和放他们的位置分别是 `fromi` 和 `toi` 。这些位置是从汽车的初始位置向东的公里数。
       当且仅当你可以在所有给定的行程中接送所有乘客时，返回 true，否则请返回 false

  条件总结:
    1. from: 上车位置, 包含, to: 下车位置, 不包含

  提示:
    1. 1 <= trips.length <= 1000
    2. trips[i].length == 3
    3. 1 <= numPassengersi <= 100
    4. 0 <= fromi < toi <= 1000
    4. 1 <= capacity <= 10的5次方
*/

/*!
  1109. 航班预订统计
  地址: https://leetcode.cn/problems/corporate-flight-bookings/description/
  问题: 这里有 `n` 个航班，它们分别从 `1 `到 `n` 进行编号
       有一份航班预订表 `bookings`, 表中第 `i` 条预订记录 `bookings[i] = [firsti, lasti, seatsi]` 意味着在从 `firsti` 到 `lasti` （包含 firsti 和 lasti ）的 `每个航班` 上预订了 `seatsi` 个座位
       请你返回一个长度为 `n` 的数组 `answer`，里面的元素是每个航班预定的座位总数。
  提示:
    1. 1 <= n <= 2 * 10的4次方
    2. 1 <= bookings.length <= 2 * 10的4次方
    3. bookings[i].length == 3
    4. 1 <= firsti <= lasti <= n
    5. 1 <= seatsi <= 104的4次方
*/

pub struct NumArray {
    diff: Vec<i32>,
    nums: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        if nums.is_empty() {
            return Self {
                diff: Vec::new(),
                nums: Vec::new(),
            };
        }

        let len = nums.len();
        let mut diff = vec![0; len + 1];
        for i in 1..len {
            diff[i] = nums[i] - nums[i - 1];
        }

        Self { diff, nums }
    }

    pub fn increment(&mut self, left: i32, right: i32, inc: i32) {
        if self.diff.is_empty() {
            return;
        }

        self.diff[left as usize] += inc;

        let mut r = (right + 1) as usize;
        if r >= self.diff.len() {
            r = self.diff.len() - 1;
        }

        self.diff[r] -= inc;
    }

    // 还原数组
    pub fn restore(&self) -> Vec<i32> {
        if self.nums.is_empty() || self.diff.is_empty() {
            return Vec::new();
        }

        let mut results = Vec::with_capacity(self.nums.len());
        results.push(self.diff[0]); // A[0] = D[0]

        for i in 1..self.nums.len() {
            // A[i] = A[i-1] + D[i]
            results.push(results[i - 1] + self.diff[i]);
        }

        results
    }
}

// 拼车问题
pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let nums = vec![0; 1001];
    let mut diff = NumArray::new(nums);

    if diff.diff.is_empty() || trips.is_empty() || diff.nums.is_empty() {
        return false;
    }

    for trip in trips {
        let num = trip[0];
        let left = trip[1];
        let right = trip[2];
        diff.increment(left, right - 1, num);
    }

    // 还原, 检查是否超载
    let arr = diff.restore();
    for a in arr {
        if a > capacity {
            return false;
        }
    }

    true
}

// 航班预定
pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let nums = vec![0; n as usize];
    let mut diff = NumArray::new(nums);

    if diff.diff.is_empty() || bookings.is_empty() || diff.nums.is_empty() {
        return Vec::new();
    }

    for booing in bookings {
        let left = booing[0];
        let right = booing[1];
        let num = booing[2];
        if num > 0 {
            diff.increment(left - 1, right - 1, num);
        }
    }

    diff.restore()
}
