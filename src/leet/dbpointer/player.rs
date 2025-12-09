/*!
  2410. 运动员和训练师的最大匹配数
  力扣: https://leetcode.cn/problems/maximum-matching-of-players-with-trainers/description
  题目: 给你一个下标从 0 开始的整数数组 players ，其中 players[i] 表示第 i 名运动员的 能力 值，同时给你一个下标从 0 开始的整数数组 trainers ，其中 trainers[j] 表示第 j 名训练师的 训练能力值 。
       如果第 i 名运动员的能力值 小于等于 第 j 名训练师的能力值，那么第 i 名运动员可以 匹配 第 j 名训练师。除此以外，每名运动员至多可以匹配一位训练师，每位训练师最多可以匹配一位运动员。
       请你返回满足上述要求 players 和 trainers 的 最大 匹配数。

  示例1: 输入: players = [4,7,9], trainers = [8,2,5,8]
        输出: 2
        解释:
        得到两个匹配的一种方案是：
        - players[0] 与 trainers[0] 匹配，因为 4 <= 8
        - players[1] 与 trainers[3] 匹配，因为 7 <= 8
        可以证明 2 是可以形成的最大匹配数

  示例2: 输入: players = [1,1,1], trainers = [10]
        输出: 1
        解释:
        训练师可以匹配所有 3 个运动员
        每个运动员至多只能匹配一个训练师，所以最大答案是 1

  解: 排序 + 双指针(贪心)
      1. 使用两个指针:i, j, 分别扫描 players 和 trainers
      2. 如果 trainers[j] 能训练 players[i], 如果匹配成功, 则 i++, j++, count++
      3. 否则 trainers[j] 太弱, j++
      因为:
      - 最弱的运动员尽量由最弱能训练他的训练师匹配
      - 不浪费强训练师在适配范围太大的运动员上(贪心最优)

  时间复杂度: O(n log n + m log m)
  空间复杂度: O(1) 原地排序
*/
pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
    if players.is_empty() || trainers.is_empty() {
        return 0;
    }

    let mut players = players.clone();
    let mut trainers = trainers.clone();

    players.sort(); // players.sort_by(|a, b| a.cmp(b));
    trainers.sort();

    let mut i = 0i32;
    let mut j = 0i32;
    let mut count = 0;

    let m = players.len() as i32;
    let n = trainers.len() as i32;

    while i < m && j < n {
        if players[i as usize] <= trainers[j as usize] {
            count += 1;
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    count
}
