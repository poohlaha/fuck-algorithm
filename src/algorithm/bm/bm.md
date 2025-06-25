# Boyer-Moore 算法
  简称 `BM` 算法, 是 `一种用于字符串匹配的经典算法`, 它被广泛认为是 `实际应用中效率最高的字符串搜索算法之一`，尤其 `适合在大文本中搜索较短的模式串`
  
1. 特性
   - 与 `KMP` 算法不同的是，`KMP` 每次 `只能移动一位或根据部分匹配表小步前进`, 而 `BM` 在不匹配时可以 `跳跃多个字符`
   - 在 `主串 text` 中查找 `模式串 pattern` 时，`从右往左` 对模式串进行比较, 利用不 `匹配时的启发式规则(Bad Character 和 Good Suffix)` 进行 `大步跳跃`，从而减少不必要的比较
   
2. 两大启发式规则
   - 坏字符规则(Bad Character Rule)
     - 当字符不匹配时，`利用当前不匹配字符在模式串中最后出现的位置` 来决定模式串向右移动的距离
     ```text
     text: HERE IS A SIMPLE EXAMPLE
     pattern: EXAMPLE
   
     1. 当前比较到 `text[i] = I` 与 `pattern[j] = A` 不匹配
     2. `A` 在模式串中出现的位置是 `pattern[2]`
     3. 所以可以路过一定距离, 使用 `pattern[2]`` 对齐 `text[i]`(或更远)
     ```
     
   - 好后缀规则(Good Suffix Rule)
     当模式串的一部分后缀匹配成功，但之后出现不匹配，`利用已匹配成功的后缀信息`，将模式串向右移动到 `下一个相同后缀的位置`
     - 从 `后向前遍历`
       - `BM` 匹配时是 `从右往左` 匹配的，所以构建好后缀规则时，更自然从模式串靠近尾部的字符开始向前查看匹配关系
       - 可以及时捕捉最长公共后缀与前缀的信息，方便构造好后缀跳转表

3. 时间复杂度
   - 最好情况: O(n/m)(非常高效, 跳跃多)
   - 平均情况: O(n)
   - 最坏情况: O(n × m)(这种情况极少出现)

4. 空间复杂度
   - 需要预处理两个表: `坏字符表` 和 `好后缀表`
   - 空间复杂度为: `O(m + σ)`, 其中 `σ` 是字符集大小(如 ASCII 为 256)

5. 适用场景
   - 文本编辑器中的查找功能
     - 如: Vim、Sublime、VSCode 的搜索
   - 字符串搜索引擎或语法高亮器中
   - 当:
     - 模式串较短(几个个字符以内)
     - 文本串较长(成千上万个字符)
     - 字符集大小较小(如英文)

6. 步骤
   - 预处理阶段
     - 构建坏字符规则表
       - 当模式串和文本字符 `不匹配` 时，找到文本中 `坏字符在模式串中最后出现的位置`，据此决定跳跃多少
       - 对模式串 `pattern` 中的每个字符, 记录它在模式串中 `最后出现的位置`
       - 如果某字符不在模式串中, 记录为 `-1`
       - 这个表用来指导当发生坏字符不匹配时, 模式串向右滑动的距离
     - 构建好后缀规则表
       - 好后缀规则是当后缀匹配成功但出现坏字符时，模式串移动的距离
       - 包含两部分
         - suffix 数组: 记录模式串中后缀子串在模式串中其他位置出现的起始索引(除末尾外), 没有出现记为 `-1`
           - suffix 数组 key: 匹配到的字符数, value: 匹配字符最开始索引
           - 长度为 `k` 的 `好后缀`，在模式串前面从 `suffix[k] 位置开始出现过`
         - prefix 数组: 记录后缀子串是否是模式串的前缀(`true` 或 `false`)
           - prefix 数组 key: 匹配到的字符数, value: 是不是匹配到了开头
           - 为 `true`, 说明: 模式串的 `前 k 个字符` 和 `末尾长度为 k` 的 `后缀相同`(`前缀 = 后缀`)
       - 计算步骤
         - 从后往前遍历模式串, 计算所有后缀的最长匹配位置是否匹配前缀
         - 这个表用来决定当坏字符出现时, 基于已经匹配的后缀, 模式串应该移动多少
   - 匹配阶段(从 `右向左` 比对 `pattern` 和 `text` 的窗口，失败就使用 `坏字符` 和 `好后缀` 决定下一步滑动距离)
     - 在主串中, 将模式串和主串对齐
       - 取出 `text[0..m]`(`m` 为 `pattern` 的 `长度 - 1`) 与 `pattern` 比较
     - `从模式串末尾开始向前匹配` 字符
     - 如果遇到坏字符(不匹配), 则:
       - 根据 `坏字符规则` 计算坏字符跳转距离 `x` (发生匹配的索引位置 - 当前字符在坏字符表中的索引)(这个值就是跳过的值)
         - 直接移动到最后一个发生匹配字符的位置, 然后跳过这个位置到最后的几个字符, 以对齐下一次继续对比
         ```text
          pattern 长度为 m
          如果匹配发现 text[3] 和 pattern[6] 不匹配
          1. 查找 text[3] 对应字符在 pattern 中最后一次出现的位置 A
          2. 直接滑动到 pattern 最后: j - A(后续比较)(这里不需要 + 1, 是因为保证 text[3] 字符和位置 A 上字符对齐)
          继续下一次比较
         ```
       - 根据 `好后缀规则` 计算好后缀跳转距离 `y` (最后一个字符的索引 - 发生匹配的索引位置)(这个值就是匹配的值, 前面有多少字符匹配成功)
         - 如果坏字符中有部分匹配, 获取到匹配字符, 然后拉过来对齐
         ```text
         好后缀长度 = k = m - 1 - j(最后一个字符的索引 - 发生匹配的索引位置)
         1. 查找 suffix[k] !== -1, 满足则: y = j - suffix[k] + 1, 不满足接着走 2(这里 + 1, 是因为向前滑动 suffix[k] 个字符后对齐)
         2. 查找 prefix[k] == true, 满足则: y = pattern 长度 m - k, 不满足接着走 3
         3. 以上条件全部不满足: y = m
         ```
       - 取较大值 `max(x, y)`, 将模式串 `右移对应步数`
     - 如果完全匹配, 则匹配成功, 进一位接着匹配(如果只需要匹配一次, 则直接返回匹配位置)(i += 1)

7. 举例
   ```text
   假设:
   text: TTTABABACABAABACABAXABACABAABACABAY
   T T T A B A B A C A  B  A  A  B  A  C  A  B  A  X  A  B  A  C  A  B  A  A  B  A  C  A  B  A  Y
   0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34
   pattern: ABABACABA
            A B A B A C A B A
            0 1 2 3 4 5 6 7 8 
   
   模式串长度 m: 9
   
   1. 预处理阶段
   1.1 构建坏字符规则表
   根据 pattern 中每个字符构建坏字符规则表 H
   H['A'] = 0  -- A 第 1 次出现
   H['B'] = 1  -- B 第 1 次出现
   H['A'] = 2  -- A 第 2 次出现, 更新 A 的索引
   H['B'] = 3  -- B 第 2 次出现, 更新 B 的索引
   H['A'] = 4  -- A 第 3 次出现, 更新 A 的索引
   H['C'] = 5  -- C 第 1 次出现
   H['A'] = 6  -- A 第 4 次出现, 更新 A 的索引
   H['B'] = 7  -- B 第 3 次出现, 更新 B 的索引
   H['A'] = 8  -- A 第 5 次出现, 更新 A 的索引
   
   最终
   H['A'] = 8
   H['B'] = 7
   H['C'] = 5
   其它字符的坏字符表值 = -1(表示不在模式串中)
   
   1.2 构建好后缀规则表
   1.2.1 构建 `suffix 数组` 和 `prefix 数组`
   索引为[0, 8] 去除末尾后: [0, 7]
   遍历 i(从 i = 7 向前遍历到 i = 0), 把 pattern[i, m - 1] 作为一个后缀与模式串前缀比较
   
   suffix = [-1, -1, -1, -1, -1, -1, -1, -1, -1]
   prefix = [false, false, false, false, false, false, false, false, false]
   i = 7, j = i, k = 0
   
   i = 7
   -> j = i = 7, k = 0
   -> 比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[7] = B, pattern[8 - 0] = pattern[8] = A
   -> 不相等, 没有任何后缀匹配成功, suffix 和 prefix 不变
   
   i = 6
   -> j = i = 6, k = 0
   -> 比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[6] = A, pattern[8 - 0] = pattern[8] = A
   -> 匹配成功一个长度为 1 的后缀 `A`
   -> j -= 1 = 5, k += 1 = 1
   -> 继续比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[5] = C, pattern[8 - 1] = pattern[7] = B
   -> 不相等
   -> 总共匹配到一个长度为 1 的后缀 `A
   -> 没有匹配到开头, prefix 不变, 更新 suffix[1] = j + 1 = 6(表示: 长度为 1 的后缀 `A`，在 pattern 的第 `j` 个位置出现过)
   -> pattern[6] = A 与 pattern[8] = A 完全一样
   
   i = 5
   -> j = i = 5, k = 0
   -> 比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[5] = C, pattern[8 - 1] = pattern[7] = B
   -> 不相等, 没有任何后缀匹配成功, suffix 和 prefix 不变
   
   i = 4
   -> j = i = 4, k = 0
   -> 比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[4] = A, pattern[8 - 1] = pattern[8] = A
   -> 匹配成功一个长度为 1 的后缀 `A`
   -> j -= 1 = 3, k += 1 = 1
   -> 继续比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[3] = B, pattern[8 - 1] = pattern[7] = B
   -> 匹配成功一个长度为 1 的后缀 `B`
   -> j -= 1 = 2, k += 1 = 2
   -> 继续比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[2] = A, pattern[8 - 2] = pattern[6] = A
   -> 匹配成功一个长度为 1 的后缀 `A`
   -> j -= 1 = 1, k += 1 = 3
   -> 继续比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[1] = B, pattern[8 - 3] = pattern[5] = C
   -> 不相等
   -> 总共匹配到一个长度为 2 的后缀 `A, 一个长度为 1 的后缀 `B`
   -> 没有匹配到开头, prefix 不变, 更新 suffix[3] = j + 1 = 2
   -> pattern[2..4] = ABA 与 pattern[6..8] = ABA 完全一样
   
   i = 3
   -> j = i = 3, k = 0
   -> 比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[3] = B, pattern[8 - 1] = pattern[8] = A
   -> 不相等, 没有任何后缀匹配成功, suffix 和 prefix 不变
   
   i = 2
   -> j = i = 2, k = 0
   -> 比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[2] = A, pattern[8 - 1] = pattern[8] = A
   -> 匹配成功一个长度为 1 的后缀 `A`
   -> j -= 1 = 1, k += 1 = 1
   -> 继续比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[1] = B, pattern[8 - 1] = pattern[7] = B
   -> 匹配成功一个长度为 1 的后缀 `B`
   -> j -= 1 = 0, k += 1 = 2
   -> 继续比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[0] = A, pattern[8 - 2] = pattern[6] = A
   -> 匹配成功一个长度为 1 的后缀 `A`
   -> j -= -1, k += 3
   -> 此时 j < 0, 匹配到了开头 
   -> 总共匹配到一个长度为 2 的后缀 `A, 一个长度为 1 的后缀 `B`
   -> 匹配到开头, prefix[3] = true, 更新 suffix[3] = j + 1 = 0
   -> pattern[0..2] = ABA 与 pattern[6..8] = ABA 完全一样
   
   i = 1
   -> j = i = 1, k = 0
   -> 比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[1] = B, pattern[8 - 1] = pattern[8] = A
   -> 不相等, 没有任何后缀匹配成功, suffix 和 prefix 不变
   
   i = 0
   -> j = i = 0, k = 0
   -> 比较 pattern[j] 和 pattern[8 - k] 是否相等
   -> pattern[0] = A, pattern[8 - 1] = pattern[8] = A
   -> 匹配成功一个长度为 1 的后缀 `A`
   ->  j -= -1, k += 1
   -> 此时 j < 0, 匹配到了开头 
   -> 总共匹配到一个长度为 1 的后缀 `A`
   -> 匹配到开头, prefix[1] = true, 更新 suffix[1] = j + 1 = 0
   -> pattern[0] = A 与 pattern[8] = A 完全一样
   
   最终: 
   suffix[1] = 0 // "A" 出现在最开头
   suffix[3] = 0 // "ABA" 既是后缀也是前缀
   其余为 -1, suffix = [-1, 0, -1, 0, -1, -1, -1, -1, -1]
   
   prefix[1] = true    // "A"
   prefix[3] = true    // "ABA"
   其余为 false, prefix = [false, true, false, true, false, false, false, false, false]
   
   2. 匹配阶段(从 `右向左` 比对 `pattern` 和 `text` 的窗口，失败就使用 `坏字符` 和 `好后缀` 决定下一步滑动距离)
   2.1 在主串中, 将模式串和主串对齐
   
   i = 0(T T T A B A B A C)
   -> 取出 text[0..8] 与 pattern 比较
   -> 从右向左比较每个字符
   -> 对比 text[8] 和 pattern[8], text[8] = C, pattern[8] = A ❌
   -> 匹配失败, 触发坏字符和好后续规则
   -> 坏字符是 C(text[8]), 查坏字符表 H : H['C'] = 5(pattern 中最后一个 C 出现在 5), 当前失误发生在 pattern[8]
   -> 移动距离 = 8 - H['C'] = 8 - 5 = 3
   -> 查看 prefix[8] = -1, 没有好后缀
   -> 滑动距离: 3, i += 3 = 3
   
   i = 3(A B A B A C A  B  A)
   -> 取出 text[3..11] 与 pattern 比较
   -> 从右向左比较每个字符
   -> 对比 [text11] 与 pattern[8], text[11] = A, pattern[8] = A ✅
   -> 对比 [text10] 与 pattern[7], text[10] = B, pattern[7] = B ✅
   -> 对比 [text9] 与 pattern[6], text[9] = A, pattern[6] = A ✅
   -> 对比 [text8] 与 pattern[5], text[8] = B, pattern[5] = C ✅
   -> 对比 [text7] 与 pattern[4], text[7] = A, pattern[4] = A ✅
   -> 对比 [text6] 与 pattern[3], text[6] = B, pattern[3] = B ✅
   -> 对比 [text5] 与 pattern[2], text[5] = A, pattern[2] = A ✅
   -> 对比 [text4] 与 pattern[1], text[4] = B, pattern[1] = B ✅
   -> 对比 [text3] 与 pattern[0], text[3] = A, pattern[0] = A ✅
   -> 完全匹配, i += 1
   
   i = 4(B A B A C A  B  A  A)
   -> 取出 text[4..12] 与 pattern 比较
   -> 从右向左比较每个字符
   -> 对比 [text12] 与 pattern[8], text[12] = A, pattern[8] = A ✅
   -> 对比 [text11] 与 pattern[7], text[11] = A, pattern[7] = B ❌
   -> 匹配失败, 触发坏字符和好后续规则
   -> 坏字符为 A(text[11]), 查坏字符表 H : H['A'] = 8(pattern 中最后一个 A 出现在 8), 当前失误发生在 pattern[7]
   -> 坏字符规则移动距离 x = 7 - H['A'] = 7 - (8) = -1
   -> 匹配好后续规则
   -> 好后续规则1: 查找 suffix[k] 是否为 -1, suffix[8 - 7] = suffix[1] = 0, 满足: y = j - suffix[k] + 1 = 7 - 0 + 1 = 8
   -> max(x, y) = max(-1, 8) = 8
   -> 滑动距离: 8, i += 8 = 13
   
   i = 13(13 14 15 16 17 18 19 20 21)
   -> 取出 text[13..21] 与 pattern 比较
   -> 从右向左比较每个字符
   -> 对比 [text21] 与 pattern[8], text[21] = C, pattern[8] = A ❌
   -> 匹配失败, 触发坏字符和好后续规则
   -> 坏字符是 C(text[21]), 查坏字符表 H : H['C'] = 5(pattern 中最后一个 C 出现在 5), 当前失误发生在 pattern[8]
   -> 坏字符规则移动距离 x = 8 - H['C'] = 8 - 5 = 3
   -> 匹配好后续规则
   -> 好后续规则1: 查找 suffix[k] 是否为 -1, suffix[8 - 8] = suffix[0] = -1, 不满足
   -> 好后续规则2: 查找 prefix[k] 是否为 true, prefix[8 - 8] = false, 不满足
   -> 好后续规则3: y = m, y = 9
   -> max(x, y) = max(3, 8) = 9
   -> 滑动距离: 9, i += 9 = 22
   
   i = 22(A  C  A  B  A  A  B  A  C)
   -> 取出 text[22..30] 与 pattern 比较  
   -> 从右向左比较每个字符
   -> 对比 [text30] 与 pattern[8], text[30] = C, pattern[8] = A ❌
   -> 匹配失败, 触发坏字符和好后续规则
   -> 坏字符是 C(text[30]), 查坏字符表 H : H['C'] = 5(pattern 中最后一个 C 出现在 5), 当前失误发生在 pattern[8]
   -> 坏字符规则移动距离 x = 8 - H['C'] = 8 - 5 = 3
   -> 匹配好后续规则
   -> 好后续规则1: 查找 suffix[k] 是否为 -1, suffix[8 - 8] = suffix[0] = -1, 不满足
   -> 好后续规则2: 查找 prefix[k] 是否为 true, prefix[8 - 8] = false, 不满足
   -> 好后续规则3: y = m, y = 9
   -> max(x, y) = max(3, 8) = 9
   -> 滑动距离: 9, i += 9 = 31
   
   i = 31(A  B  A  Y)
   -> 31 + 8 = 39, 超出边界  ❌
   -> 匹配结束
   ```