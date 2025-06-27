# Quick Search(QS)
  `QS` 算法是一种 `启发式的字符串匹配算法`, 结合了 `BM` 算法的 `坏字符规则的一种改进`
  
1. 特性
   - 总是看 `模式串(pattern)` `右边的一个字符`(称为 `观察字符`)
   - 如果当前窗口 `无法匹配`，就根据 `观察字符` 决定 `下一次滑动的距离`
   - 每次对比 `从左向右(BM 是从右向左)`
   - 失败后根据观察字符移动, `不回退`
   
2. 时间复杂度
   - 预处理: O(m)(构建 shift 表)
   - 匹配过程: 平均 O(n/m)(取决于跳跃步长)
   - 最坏情况: O(n × m)(但几乎不会出现, 除非重复字符很多)

3. 空间复杂度
   - O(σ)(其中 `σ` 是字符集大小(如 ASCII 为 256))

4. 实现步骤
   - 创建一个大小为字母表(ASCII 256) 的数组 `shift[]`, 全部初始化为 `pattern.len() + 1`, `shift[]取第一次出现的索引位置`
     - 字符在 `pattern` 中 `越靠右`，`shift 值越小`
     - 字符在 `pattern` 中 `越靠左`，`shift 值越大`
   - 遍历模式串 `pattern`, 对于每个字符 `pattern[i]`, 设置: `shift[pattern[i]] = pattern.len() - i`(`pattern[i]` 出现 `越靠右`，`偏移越小`)
   - 进行匹配
     - 从主串 `text` 的位置 `i = 0` 开始向右滑动窗口
     - 每次取出 `text[i..i+m]`(`m` 是 `pattern 长度`) `从左到右` 直接比较
     - 如果完全匹配，记录匹配位置
     - 观察 `text[i + m]`(`窗口后一位字符`)
     - 用这个字符查 `shift[c]`，然后让 `i += shift[c]`
     - 如果 `i + m >= text.len()`，说明剩下 `不够匹配`, 结束
     
5. 适用场景
   - 优点
     - 简单易实现
     - 平均性能优于 `KMP` 和部分 `BM` 变种
     - 没有回溯，不需要处理好后缀规则
   - 缺点
     - 不适合小字符集(如 DNA、二进制)
     - 最坏情况仍然是 O(nm)
   - 适合场景
     - 英文文本搜索
     - 多模式搜索（配合 `Sunday 算法` 变种）
     - `效率要求高`、`匹配数据偏长` 的场景

5. 举例
   ```text
   text：TTTABABACABAABACABAXABACABAABACABAY
   T T T A B A B A C A  B  A  A  B  A  C  A  B  A  X  A  B  A  C  A  B  A  A  B  A  C  A  B  A  Y
   0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34
   pattern: ABABACABA
   m: 9
   
   1. 构建 `shift` 数组
   shift = [10, 10, 10, 10, 10, 10, 10, 10, 10]
   
   从左往右填
   shift['A'] = 9 - 0 = 9  -- A 第 1 次出现
   shift['B'] = 9 - 1 = 8  -- B 第 1 次出现
   shift['A'] = 9 - 2 = 7  -- A 第 2 次出现
   shift['B'] = 9 - 3 = 8  -- B 第 2 次出现
   shift['A'] = 9 - 4 = 5  -- A 第 3 次出现
   shift['C'] = 9 - 5 = 4  -- C 第 1 次出现
   shift['A'] = 9 - 6 = 3  -- A 第 4 次出现
   shift['B'] = 9 - 7 = 2  -- B 第 3 次出现
   shift['A'] = 9 - 8 = 1  -- A 第 5 次出现
      
   
   最终:
   shift['A'] = 1
   shift['B'] = 2
   shift['C'] = 4
   其他字符 = 10（默认）
   
   2. 匹配过程
   从主串 `text` 的位置 `i = 0` 开始向右滑动窗口
   
   i = 0
   -> 窗口: text[i, m - 1] = text[0, 8] = "TTTABABAC"
   -> 比较 `窗口字符` 和整个 `pattern` 是否匹配
   -> "TTTABABAC" != "ABABACABA" ❌
   -> 观察字符 `i + m` 为 'A', 查找 `shift` 数组是否存在
   -> 存在, shift['A'] = 1
   -> 更新 i, i += 1 = 1
   
   i = 1
   -> 窗口: text[i, m - 1] = text[1, 9] = "TTABABACA"
   -> 比较 `窗口字符` 和整个 `pattern` 是否匹配
   -> "TTABABACA" != "ABABACABA" ❌
   -> 观察字符 `i + m` 为 'B', 查找 `shift` 数组是否存在
   -> 存在, shift['B'] = 2
   -> 更新 i, i += 2 = 3
   
   i = 3
   -> 窗口: text[i, m - 1] = text[3, 11] = "ABABACABA"
   -> 比较 `窗口字符` 和整个 `pattern` 是否匹配
   -> "ABABACABA" == "ABABACABA" ✅
   -> 完全匹配, 记录位置 3
   -> 观察字符 `i + m` 为 `A`, 查找 `shift` 数组是否存在
   -> 存在, shift['A'] = 1
   -> 更新 i, i += 1 = 4
   
   i = 4
   -> 窗口: text[i, m - 1] = text[4, 12] = "BABACABAA"
   -> 比较 `窗口字符` 和整个 `pattern` 是否匹配
   -> "BABACABAA" != "ABABACABA" ❌
   -> 观察字符 `i + m` 为 'B', 查找 `shift` 数组是否存在
   -> 存在, shift['B'] = 2
   -> 更新 i, i += 2 = 6
   
   i = 6
   -> 窗口: text[i, m - 1] = text[6, 14] = "BACABAABA"
   -> 比较 `窗口字符` 和整个 `pattern` 是否匹配
   -> "BACABAABA" != "ABABACABA" ❌
   -> 观察字符 `i + m` 为 'C', 查找 `shift` 数组是否存在
   -> 存在, shift['C'] = 4
   -> 更新 i, i += 4 = 10
   
   i = 10
   -> 窗口: text[i, m - 1] = text[10, 18] = "BAABACABA"
   -> 比较 `窗口字符` 和整个 `pattern` 是否匹配
   -> "BAABACABA" != "ABABACABA" ❌
   -> 观察字符 `i + m` 为 'X', 查找 `shift` 数组是否存在
   -> 不存在, 取 shift 默认值 10
   -> 更新 i, i += 10 = 20
   
   i = 20
   -> 窗口: text[i, m - 1] = text[20, 28] = "ABACABAAB"
   -> 比较 `窗口字符` 和整个 `pattern` 是否匹配
   -> "ABACABAAB" != "ABABACABA" ❌
   -> 观察字符 `i + m` 为 `A`, 查找 `shift` 数组是否存在
   -> 存在, shift['A'] = 1
   -> 更新 i, i += 1 = 21
   
   i = 21
   -> 窗口: text[i, m - 1] = text[21, 29] = "BACABAABA"
   -> 比较 `窗口字符` 和整个 `pattern` 是否匹配
   -> "BACABAABA" != "ABABACABA" ❌
   -> 观察字符 `i + m` 为 `C`, 查找 `shift` 数组是否存在
   -> 存在, shift['C'] = 4
   -> 更新 i, i += 4 = 25
   
   i = 25
   -> 窗口: text[i, m - 1] = text[25, 33] = "BAABACABA"
   -> 比较 `窗口字符` 和整个 `pattern` 是否匹配
   -> "BAABACABA" != "ABABACABA" ❌
   -> 观察字符 `i + m` 为 `Y`, 查找 `shift` 数组是否存在
   -> 不存在, 取 shift 默认值 10
   -> 更新 i, i += 10 = 35
   
   i = 35
   -> 越界, 匹配结束
   
   最终:
   找到一次匹配，位置在 i = 3
   ```

5. 与 `BM` 和 `BM-Horspool` 对比
   ```text
   特性                     QS                             BM                                  BM-Horspool
   全称                     Quick Search                   Boyer-Moore                         Boyer-Moore-Horspool
   匹配方向                  ⬅️ 从左往右                     ⬅️ 从右往左                          ⬅️ 从右往左
   表类型                   shift 表（基于text[i + m]）      坏字符表 + 好后缀表                    坏字符表(简化版)
   使用哪个字符跳跃           text[i + m]                    text[i + j](当前 mismatched 字符)     text[i + m - 1](即 pattern 最末字符)
   坏字符规则                ✅(简化为后一位)                 ✅(记录每个字符 `最后出现位置`)         ✅(记录每个字符 `最后出现位置`) 
   好后缀规则                ❌ 无                          ✅(最大好后缀 + border prefix)        ❌ 无
   预处理复杂度              O(m)                            O(m + σ)                             O(m + σ)
   匹配最坏复杂度            O(n × m)                        O(n × m)(最坏), O(n)(平均)             O(n × m)
   平均匹配性能              ✅ 快(跳跃靠后字符，效果稳定)       ✅ 非常快(跳跃大但实现复杂)             ✅ 快(实现简洁)
   实现复杂度                ⭐ 最简单                        ❌ 最复杂(需理解两个规则)              ⭐⭐ 中等
   支持 Unicode             ✅ (推荐用 Vec<char>)            ✅ (需要额外字符处理)                  ✅ (需要额外字符处理) 
   实际使用场景              编辑器、全文搜索、快速匹配等         极限性能场景，如搜索引擎、DNA 序列匹配等   实用简洁，适用于大多数通用匹配任务
   ``` 
   
6. 使用建议
   ```text
   场景                                  推荐算法                       原因说明
   快速上手 / 只要查找结果                  Quick Search ✅              实现最简单, 不牺牲太多性能
   对性能要求极高(搜索引擎、海量匹配)         BM 💪                        跳跃最远，平均性能最好，但实现最复杂 
   想要好性能又不想写好后缀表                BM-Horspool ✨               BM 的简化变种，性能好，逻辑好写
   模式串长，文本也大                       BM 或 BM-Horspool            模式串越长, 这类算法跳跃越远、优势越大
   中文、emoji混合内容                     任意(建议 QS)                  推荐用 Vec<char>，QS 实现最友好
   ```