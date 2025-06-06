/*!
  贪心算法
*/

pub struct Greedy;

#[derive(Debug, Default, Clone)]
pub struct Meeting {
  pub start: u32,
  pub end: u32
}
impl Greedy {

  // 1. 会议室预约, 假设有一间会议室，有若干场不同的会议申请使用，每场会议有自己的开始时间和结束时间
  // 目标: 安排尽可能多的会议，且同一时间只能有一个会议进行(互不重叠)
  // 每次选择 `结束时间最早的会议`, 这样为后面的会议留出更多时间，局部最优解往往也导致全局最优
  pub fn max_non_overlapping_meetings(meeings: &mut Vec<Meeting>) -> Vec<Meeting> {
    // 按结束时间升序排列
    meeings.sort_by(|a, b| a.end.cmp(&b.end));

    let mut results: Vec<Meeting> = Vec::new();
    let mut last_meeting_end_time: u32 = 0;

    for meeting in meeings .iter(){
      if meeting.start > last_meeting_end_time {
        last_meeting_end_time = meeting.end;
        results.push(meeting.clone())
      }
    }

    results
  }

  // 2. 以 `最小硬币找零` 为例, 假设有一些不同面额的硬币，目标是`凑成一个指定金额`，且使用的硬币数目 `尽可能少`
  // 每次选择当前 `最大且不超过所需金额` 的面额, 直接凑成总金额
  pub fn coin_change(amount: u32, coins: &mut Vec<u32>) -> Vec<u32> {
    // 按面额从小到大考虑
    coins.sort_by(|a, b| b.cmp(a));

    let mut remaining = amount;
    let mut results: Vec<u32> = Vec::new();
    for coin in coins.iter() {
       while remaining >= coin.clone() {
         remaining -= coin.clone();
         results.push(coin.clone());
       }
    }

    results
  }
}