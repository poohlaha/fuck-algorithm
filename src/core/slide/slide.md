# 滑动窗口算法
  滑动窗口算法是一种在 `连续数据序列`(如数组、字符串等)中，使用 `一个窗口(通常是固定大小或者可以变化的范围)` 来解决特定问题的技巧
  - 窗口在序列上向前滑动，期间根据窗口内的元素更新答案，避免了每次都重复计算，提高效率
  - 可以把它想象成用一个 `尺子` 在序列上滑动，动态维护当前尺子覆盖的内容

1. 核心思想
   - 定义窗口: 窗口是一个区间, 通常由两个指针 `left` 和 `right` 表示区间的两端
   - 窗口滑动: 窗口向前滑动意味着 `right` 向右移动, 或者 `left` 向右收缩窗口
   - 窗口内状态更新: 在窗口滑动时, 动态维护窗口内的状态(比如当前和、最大值、最小值、元素种类等)
   - 只遍历一次序列, 避免重复计算
   - `只在需要时移动窗口边界, 从而避免重复遍历窗口内所有元素`

2. 特性
   - 只需要一次遍历序列(时间复杂度为 O(n))
   - 通过双指针灵活扩展/收缩窗口，减少冗余计算
   - 适合 `连续子数组`、`子串` 等 `线性数据结构` 上的问题
   - 窗口可以是 `固定大小`，也可以是 `动态变化的大小(可变窗口)`
   - 适用范围
       - 在 `连续子串/子数组` 问题中高效寻找
           - 固定长度窗口(长度 `k`)
           - 不定长度窗口(最小/最大满足条件窗口)
       - 常用于
           - 子数组最大/最小和
           - 字符串匹配中的窗口枚举(如 `Rabin-Karp`、`BM` 等)
           - 最长无重复子串、最短覆盖子串
           - 求最大/最小值、和、频率等窗口内统计
   
3. 时间复杂度
    - 一般为 `O(n)`
    - 因为窗口左右端最多移动 `n` 次
    - 某些情况下
        - 需要窗口内部做额外操作时可能有额外 `O(log k)`(如: 窗口内求最大值时用单调队列/堆)

4. 空间复杂度
    - 一般为 `O(k)`(窗口长度)
    - 如果需要维护哈希表/计数表
        - 若字符集/元素有限, 空间通常是 `O(σ)(σ = 字符集大小)`
        - 若动态维护元素出现次数, 空间取决于不同元素的数量

5. 使用场景
    - 网络传输 / 数据流处理
        - TCP 流量控制
            - TCP 会话通过滑动窗口控制数据包的发送和确认范围，防止拥塞
            - 窗口大小动态调整，根据网络状态滑动
            - 实际例子
                - 在 `高吞吐量文件传输` 中，通过滑动窗口确保可以持续发送一定范围内未确认的数据包
        - 实时数据流统计
            - 对实时日志流、loT设备数据流，持续计算:
                - 最近 1 分钟平均值
                - 最近 1 分钟最大值/最小值
                - 最近 N 条数据统计
            - 实际例子
                - 实时监控温度传感器，计算过去 10 秒的最高温度
    - 金融领域
        - 股票价格移动平均
            - 股票图表常见的 `MA`(Moving Average 移动平均线)
                - 5 日 / 10 日 / 30 日 均线
            - 本质是维护一个长度固定的滑动窗口求和 / 求平均
        - 高频交易
            - 实时检测过去 N 秒内的交易频率、成交量极值
            - 检测价格波动是否超过阈值(如最近 1000 条交易内最大价格差)
    - 系统监控
        - 服务器健康检测
            - CPU 使用率过去 1 分钟平均值超过 80% 告警
            - 请求响应延迟的滑动窗口最大值监控
            - 在 Prometheus、Datadog 等监控系统中常见
    - 图像/视频处理
        - 图像滤波(滑动窗口卷积)
            - 图像中应用 卷积核在图像像素矩阵上滑动，计算局部平均、加权平均、边缘检测等
            - 视频中处理每一帧时，可维护滑动窗口做帧间降噪、运动检测
    - 安全领域
        - 恶意流量检测
            - 在流量包序列中，维护过去 N 包数据特征滑动窗口，检测是否出现恶意模式
                - 检测异常登录行为
                    - 滑动窗口检测过去 10 分钟登录次数是否超过阈值
    - 搜索与字符串匹配
        - 文本去重
            - `Rabin-Karp` 滑动窗口哈希用于海量文本重复检测
            - 实时流式文本(如新闻、社交平台推文)中滑动窗口匹配敏感词
        - 日志检测
            - 在实时日志流中匹配错误模式
                - 滑动窗口维护最近 N 条日志，检测是否符合某错误序列模式
    - 内存与缓存管理
        - LRU 缓存淘汰策略
            - 本质上维护滑动窗口(或近似窗口)确定最近最少使用的数据，动态淘汰旧数据
    - 数据压缩与去重
        - Rabin Fingerprinting
            - BUP、rsync、Deduplication 使用 Rabin Fingerprinting 滑动窗口分块
                - 滑动窗口固定大小计算哈希，如果哈希满足条件(如末位若干位为零)，则作为块边界
                - 实现去重和增量同步
    - 算法竞赛与刷题
        - 滑动窗口在 `LeetCode / Codeforces / Atcoder` 中常用于
            - 最长无重复子串(LC 3)
            - 长度为 K 的子数组最大和(LC 643)
            - 最短覆盖子串(LC 76)
            - 滑动窗口最大值(LC 239)
            - 判断字符串排列(LC 567)

6. 举例
   ```text
   nums = [2, 1, 5, 1, 3, 2]
   k = 3
   
   1. 初始化
   left = 0
   right = 0
   window_sum = 0
   max_sum = 0
   
   2. 窗口向右滑动
   遍历
   right = 0
   -> 加入第一个元素 nums[0] = 2
   -> window_sum += 2 = 2
   -> 窗口长度未达到 k, 继续扩张
   -> right += 1 = 1
   
   right = 1
   -> 加入第二个元素 nums[1] = 1
   -> window_sum += 1 = 3
   -> 窗口长度未达到 k, 继续扩张
   -> right += 1 = 2
   
   right = 2
   -> 加入第三个元素 nums[2] = 5
   -> window_sum += 5 = 8
   -> 窗口长度达到 k(k = 3)
   -> 更新最大值 max_sum = max(0, 8) = 8
   -> 收缩窗口左端: left += 1 = 1
   -> 移除左边第一个元素: window_sum -= 2 = 6
   -> right += 1 = 3
   
   right = 3
   -> 加入第四个元素 nums[3] = 1
   -> window_sum += 1 = 7
   -> 窗口长度达到 k(k = 3)
   -> 更新最大值 max_sum = max(8, 7) = 8
   -> 收缩窗口左端: left += 1 = 2
   -> 移除左边第一个元素: window_sum -= 1 = 6
   -> right += 1 = 4
   
   right = 4
   -> 加入第五个元素 nums[4] = 3
   -> window_sum += 3 = 9
   -> 窗口长度达到 k(k = 3)
   -> 更新最大值 max_sum = max(8, 9) = 9
   -> 收缩窗口左端: left += 1 = 3
   -> 移除左边第一个元素: window_sum -= 3 = 6
   -> right += 1 = 5
   
   right = 5
   -> 加入第五个元素 nums[5] = 2
   -> window_sum += 2 = 8
   -> 窗口长度达到 k(k = 3)
   -> 更新最大值 max_sum = max(9, 8) = 9
   -> 收缩窗口左端: left += 1 = 4
   -> 移除左边第一个元素: window_sum -= 2 = 6
   -> right += 1 = 6
   
   right = 6
   -> 越界, 结束
   
   寻找数组中和为目标值的最长子数组
   假设数组: [1, 2, 1, 3, 1, 1, 1, 4], 找到和为 `5` 的最长连续子数组
   思路
     - 设立两指针: `left` 和 `right`
     - 从 `right = 0` 向开始 `向右滑动` , 每次将 `right` 对应的值加入当前和 `sum`
     - 当 `sum > 5` 时, 左侧 `left` 右移收缩窗口, 直接 `sum <= 5`
     - 每次判断 `sum == 5`, 更新最长长度
     - 最终得到最长的和为 `5` 的子数组长度
   ```

7. 与其他数据结构结合
   滑动窗口只是 `框架`，配合合适的数据结构来高效完成窗口内的统计或查询
   ```text
   配合数据结构                 用途
   队列/双端队列                窗口内维护最大值/最小值(单调队列优化)
   哈希表                      维护窗口内元素统计/出现情况(无重复子串、最短覆盖子串)
   线段树/树状数组              支持复杂区间统计/极值(若需要查询多个历史区间) 
   堆(heap)                   动态求窗口内第 K 大/小值(适合离线处理)
   滚动哈希                    窗口内字符串匹配(Rabin-Karp 等)
   ```

8. 模板
   ```text
   left = 0
   right = 0
   
    while right < nums.len() {
       # 扩张窗口，加入 nums[right]
       加入 nums[right] 到窗口状态
       right += 1
    
       while 窗口内满足条件 {
         # 在满足条件时更新答案（如更新最小长度）
         更新答案
         
         # 收缩窗口，移除 nums[left]
         移除 nums[left] 从窗口状态
         left += 1
       }
    }
   ```
