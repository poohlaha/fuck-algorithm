# Rabin Fingerprinting(拉宾指纹)
  - 用于 `数据识别、比较和重复检测的哈希算法`, 特别适合在 `滑动窗口中快速计算内容的唯一性标识`
  - 基于数学中 `多项式的有限域运算`，是一种特殊设计的 `滚动哈希算法(Rolling Hash)`，可用来处理大文件或流式数据

1. 特性
   通过数学方式为任意数据片段生成一个可滚动的指纹

2. 时间复杂度
   - 初始化一个窗口的 `fingerprint`: O(m)(m 是窗口长度)
   - 滑动时更新 fingerprint: O(1)(只需一个左移、一个减法、一个加法)

3. 空间复杂度
   - 存 `fingerprint`: O(1)
   - 如果记录所有窗口的 fingerprint 以去重，则是 `O(n - m + 1)`

4. 使用场景
   - 重复数据删除(Deduplication)
     - 在大文件中找出重复的内容块。例如:
       - 两份视频剪辑, 只有片头换了, 后面相同
       - 文件系统中备份很多相似文件
     - 把文件划分成多个块(窗口滑动)
     - 对每一块生成指纹
     - 比较指纹, 发现重复内容, 不再存储重复部分
   - 数据同步(rsync)
     - 假设你有一个 1GB 文件，只改动了中间的 5KB。你不希望重新发送整个文件
       - 服务端将旧文件分成小块，每块 `fingerprint`
       - 客户端将新文件滑动一遍，同样计算 `fingerprint`
       - 对比两边的指纹, 只发送变更块
   - 版本控制系统(如 Git)
     - Git 在比较两个提交(commit) 之间的改动时，也要判断哪些文件发生了变化
     - Git 可以用 Rabin Fingerprinting:
       - 把文件分成片段
       - 给每段算 fingerprint
       - 快速判断两次提交中，哪些片段有变化
       - 只记录变化的内容，节省存储空间
   - 网络内容缓存(CDN/Edge)
     - 在传输视频或网页资源时
       - 可用 Rabin 指纹来识别哪些片段已缓存
       - 不再重复传输已存在内容，节省带宽
       
5. 步骤
   - 同 `Rabin Karp`

6. 举例(同 `Rabin Karp`)
   ```text
   给定一个字符串 s，找出所有长度为 10 的子串中，重复出现的片段(子串)
   输入: AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT
   输出: ["AAAAACCCCC", "CCCCCAAAAA"]
   m = 10
   n = 31
   
   1. 选定参数
   p = 31 q = 10^9+7 = 1000000007
   映射数字: A = 1, C = 2, G = 3, T = 4 
   
   2. 开始匹配
   i = 0
   -> 窗口: text[i, m - 1] = text[0, 9] = "AAAAACCCCC"
   -> hash_t = (1 * 31^9 + 1 * 31^8 + 1 * 31^7 + 1 * 31^6 + 1 * 31^5 + 2 * 31^4 + 2 * 31^3 + 2 * 31^2 + 2 * 31^1 + 2 * 31^0) % 1000000007
   -> hash_t = 51609484
   -> 查找 seen 中是否存在 AAAAACCCCC
   -> 不存在, 则添加 [AAAAACCCCC, 51609484] 到 seen 中
   
   i = 1
   -> 窗口: text[i, m - 1] = text[1, 10] = "AAAACCCCCA"
   -> 根据上一个 hash 快速求出当前 hash
   -> hash_t = ((hash1 - text[i-1] * p^(m-1)) * p + 1 * p^0) % 1000000007
   -> hash_t = ((51609484 - 1 * 31^9) * 31 + 1) % 1000000007
   -> 查找 seen 中是否存在 AAAACCCCCA
   -> 不存在, 则添加 [AAAACCCCCA, xxx] 到 seen 中
   
   i = 3
   -> 窗口: text[i, m - 1] = text[2, 11] = "AAACCCCCAA"
   -> 根据上一个 hash 快速求出当前 hash
   -> hash_t = ((hash1 - text[i-1] * p^(m-1)) * p + 1 * p^0) % 1000000007
   -> hash_t = ((xxx - 1 * 31^9) * 31 + 1) % 1000000007
   -> 查找 seen 中是否存在 AAAACCCCCA

   ....   
   ```
   
7. 为什么要单独命名 `Rabin Fingerprinting`
  - 是为了抽象、复用、泛化应用场景
  - Rabin-Karp = Rabin Fingerprinting + 字符串匹配逻辑
  - Rabin Fingerprinting ≠ 只能用于 Rabin-Karp
  - 它们实现一样，但 `Rabin Fingerprinting` 这个名字，更强调它的通用性与场景独立性，而不局限在字符串匹配

8. `Rabin Karp` 和 `Rabin Fingerprinting` 对比
  ```text
  对比维度                Rabin-Karp                                      Rabin-Fingerprinting
  本质                    字符串匹配算法                                     哈希指纹技术
  核心思想                 使用滚动哈希查找某个 pattern 是否出现在主串中         用滚动哈希为数据块生成唯一指纹用于快速比对
  关键操作                 构造模式串和文本子串的 hash，滑动窗口匹配             构造每个数据块的 hash，滑动窗口生成指纹流
  输入类型                 通常是字符串(text + pattern)                      任意数据(字符串、字节块、文件、数据段)
  目标用途                 找出某个子串出现在哪些位置                          判断两段数据是否一致(通常用于 diff、去重)
  是否用滑动窗口            ✅ 是                                           ✅ 是
  是否用滚动哈希            ✅ 是                                           ✅ 是
  是否用模数(mod)q         ✅ 通常用，防止溢出 & 降低碰撞                      ✅ 也用，提升 fingerprint 质量
  是否逐字符比对            ✅ 是(hash 匹配后还需比对)                        ❌ 不比对, hash 不同直接视为不同
  典型场景                 搜索文本中的匹配子串                               比较文件版本、去重系统、Git diff、分块传输优化
  是否为通用模块            ❌ 主要是字符串算法                               ✅ 通用哈希工具，可集成到其他系统中
  是否返回位置             ✅ 是: 返回匹配位置                               ❌ 否：返回/比较 hash 值
  ```

9. 内容感知分块
   - 定义
     - 把文件切分成一系列块(chunk)，块的边界不是固定位置，而是由内容本身决定, 非常适合 `去重存储、同步、备份、增量传输`
       - 插入/删除数据不会改变后续所有块的边界
       - 同一个内容，不管位置变化，仍然会被识别为相同块
       - 非常适合去重存储、同步、备份、增量传输
   ```text
   普通分块方式是固定大小切分，比如每 1MB 为一个块。这种方式的缺点是:
   `插入、删除、移动一个字节，后面所有块的哈希都变了，命中率大幅下降`
   
   而 CDC:
   块的边界由内容本身决定，不是靠位置
   ```
   - Rabin Fingerprint 在 CDC 中的作用
     - CDC 核心是：`滑动窗口 + Rabin Fingerprint(滚动哈希)` 来寻找 `分块点`
     - 滑动窗口
       - 定义一个窗口大小 w，比如 48 字节，我们将窗口从左到右滑动整个文件，每次只移动一字节
     - Rabin Fingerprint(滚动哈希)
       - 对每个滑动窗口内容，计算一个 `Rabin Fingerprint` 值(一个数字，代表窗口内容)
       - 然后我们设定某个规则，当 fingerprint 满足这个规则，就在这个位置断开分块
       - 常见规则
         - 如果 fingerprint 的低 k 位全为 0(例如 13 位)，就认为这里是一个块的结束
       - 这样做的目的是让平均块大小 ≈ 2^k, 比如 8KB
   - 举例
   ```text
   背景: 上传一个 1GB 文件到云存储
   目录:
      1. 如果这个文件之前上传过，就跳过上传(秒传)
      2. 如果这个文件和旧文件只改了一部分，就 `只上传新块`
      3. 后台服务需要识别并 `避免重复存储`
   
   使用:
      1. CDC(内容感知分块) 来切块
      2. Rabin Fingerprinting 来决定每个块的边界
      3. 哈希(如 SHA256) 来唯一标识每个块内容
   
   假设: 有一个本地文件 movie.mp4 大小 1GB
   
   步骤:
   1. 读取文件, 进行内容感知分块
   假设参数如下:
   - 滑动窗口大小: 48 字节
   - 边界判断规则: Rabin Fingerprint 低 13 位为 0(即 % 8192 == 0)
   - 最小块大小: 4KB
   - 平均块大小: 8KB
   - 最大块大小: 16KB
   
   -> Rabin Fingerprint 会对滑动窗口内容计算 fingerprint
   -> 每次从文件中滑动窗口(每次移动 1 字节)，计算 fingerprint
   -> 如果满足规则（低13位为0），就记为块边界
   
   2. 文件分块
   整个文件被切分成大约 125,000 块（1G / 8KB ≈ 125000）
   Chunk No.	Offset(字节)  	Size (字节)	    内容摘要(哈希)
   1            0               8,112           SHA256("chunk_data_1")
   2            8,112           7,980           SHA256("chunk_data_2")
   3            7,980           8,002           SHA256("chunk_data_3")
   
   ....
   
   3. 客户端生成清单并上传
   客户端不直接上传整个文件, 而是:
   构建清单文件(manifest)
   清单中包括:
   - 块总数
   - 每个块的偏移量、大小
   - 每个块的哈希(如 SHA256)
   
   4. 后端进行对比
   后端拿到清单，检查每一个块的哈希
   - 数据库中已保存以前上传的所有块的哈希 → hash → block_id
   - 后端遍历清单中的每个块
     - 如果该块哈希在数据库中存在: 表示这个块已经上传过了，不需要再上传
     - 如果该块哈希没有: 要求客户端上传这个新块
   
   5. 客户端上传缺失的块
   - 客户端根据服务器反馈，只上传缺失的那几个块(比如10个)
   - 每个块单独上传，后端用其哈希确认内容一致性
   
   6. 后端重组文件
   后端记录这个文件的完整结构:
   - 文件 ID: abc123
   - 块序列: [chunk1_hash, chunk2_hash, ..., chunkN_hash]
   所有块存储在统一的块存储池中(类似 Git 对象)
   多个文件可能共享相同块(去重)
   ```
   - 举例
   ```text
   text1: Tucker Carlson’s two-hour interview with Ted Cruz turned out to be both more entertaining and thought-provoking than Cruz himself may have been expecting. Cruz was perhaps too literally “game” for an interview in which a fellow conservative found a friendly way to humiliate him. Specifically on the kinds of games the Trump administration is playing with foreign policy.
   text2: Tucker Carlson’s two-hour interview with Ted Cruz turned out to be both more entertaining and thought-provoking than Cruz himself may have been expecting. Cruz was perhaps too literally “game” for an interview in which a fellow conservative found a friendly way to humiliate him. Now, Specifical on the hands of games the Trump administration is playing with foreign policy.

   1. 设定参数
   窗口大小: 48KB   
   平均块大小: 4KB(实际可以变长)
   分块判断条件: Rabin fingerprint 的 末 12 位为 0(即 fingerprint & 0xFFF == 0)(0xFFF) 
   多项式基数 x: 256
   
   m = 8
   
   2. 开始匹配
   i = 0
   -> 窗口: text[i, m - 1] = text[0, 8] = "Tucker C"
   -> 转换ASCII
   ->  T   u  c   k   e   r  Space  C
   -> 84 117 99 107 101 114     32  67
   -> hash = (84 * 256^7 + 117 * 256^6 + 99 * 256^5 + 107 * 256^4 + 101 * 256^3 + 114 * 256^2 + 32 * 256^1 + 67 * 256^0) = 6029025404403847235
   -> 判断是否为边界: 6029025404403847235 & 0xFFF (非0)
   -> 不满足条件, 不是块边界
   
   i = 1
   -> 窗口: text[i, m - 1] = text[1, 9] = "ucker Ca"
   -> 转换ASCII
   ->   u  c   k   e   r  Space  C    a
   -> 117 99 107 101 114     32  67  97 
   -> hash = (hash - 84 * 256^7) * x + 97 * 256^0 = 6029025404403847235
   -> 判断是否为边界: 6029025404403847235 & 0xFFF  (非0)
   -> 不满足条件, 不是块边界
   
   ...
   
   假如在 i = 7 时 fp & 0x1F == 0, 那第切块位置是 i + m - 1 = 14
   第一个块就是文本开头第0字符到第14字符组成的一个块
   ```
   - 切块频率与掩码位数的关系
     - `掩码位数越多`, 要求指纹低位更多位为0, 满足条件的概率就越低
     - `掩码位数越少`, 条件越宽松, 满足条件的概率就越高
   - 切块的平均大小公式
     - 你用`低 k 位掩码`(即掩码是 `0x(2^k - 1)`), 那么
     - 指纹末 k 位全为0的概率是 1 / 2^k
     - 因此, 平均每 `2^k` 字节（或字符）就会出现一个 `块边界`
     - 如何确定 `k` 值
       - 根据你想要的平均块大小(例如4KB、8KB等)
       - 计算 `k = log2`(平均块大小)
       ```text
       假设想要平均 4KB 一个块
       4KB = 4096 bytes = 2^12 bytes
       则需要掩码位数 `k = 12`
       掩码就是 `低12位`，比如 `0xFFF`
       切块条件就是 fingerprint & 0xFFF == 0(指纹最低12位全0)
       ```
   - 公式
     - 字符集大小：k
     - 每字符 bits：log2k
     - 窗口长度: L
     - 哈希 bit 长度: bits * L = log2k * L
     - 最大可能组合数: k^L
     - 可能索引数组长度: 1 << (log2k * L)
     ```text
     如果不同编码的字符有: ABCDEFG
     k = 7
     窗口长度: 12
     计算 bits per char = log27 = 3, 每个字符需要 3 bits 表示
     哈希 bit 长度 = 3 * 12 = 36
     最大可能组合数 = 7^12 = 13,841,287,201 约 138亿种不同组合
     而: 1 << 36 = 68,719,476,736 约 687亿 
     ❌ 不推荐
     直接使用 HashSet 存
     
     如果不同编码的字符有: ACGT, 每个字符只需 2 bits
     窗口长度: 10
     哈希 bit 长度 = 2 * 19 = 36
     最大可能组合数 = 2^20 = 1,048,576
     可以用 1 << 20
     ```

   