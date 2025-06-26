# KMP 算法
  `KMP` 算法(`Knuth-Morris-Pratt`) 是一种 `字符串匹配算法`，用于在一个 `主串（text）` 中查找一个 `模式串（pattern）首次出现的位置`，比 `暴力匹配效率` 更高，适合大量模式匹配任务，如搜索引擎、DNA匹配等
  
1. 特性
   - `KMP` 只能用于匹配一个 `确定的字符串模式`，且它的高效之处体现在: `模式串中有可复用的前后缀结构(即字符或子串的重复)`
   - 当匹配失败时，`利用已匹配的信息`，跳过一部分字符，`不用回溯主串`
   - 预处理模式串 `pattern` 来构造一个 `部分匹配表`（又叫 `前缀函数` 或 `next 数组`），这个表告诉我们，`在匹配失败时`，`模式串可以移动多远`

2. 时间复杂度
   - 构造 `next` 数组: O(m)(m 是模式串长度)
   - 匹配过程: O(n)(n 是主串长度)
   - 总时间复杂度: O(n + m)(线性时间)
   
3. 空间复杂度
   - O(m)(存储 next 数组)

4. 步骤
   - 构造模式串的 `next` 数组(部分匹配表)
     - `next[i]` 表示 `pattern[0..i]` 这段子串的 **最长相等的真前缀和真后缀** 的长度
     - `真前缀/真后缀`: 长度不等于整个字符串本身的前缀和后缀
     - `next` 中存的是 `真前缀 = 真后缀` 的值
   - 匹配过程
     - 两指针, 一个指针 `i` 指向主串, 另一个指针 `j` 指向模式串, 初始为 `0`
     - 比较 `text[i]` 和 `pattern[i]` 是否匹配
       - `匹配` `i += 1`, `j += 1`
       - `不匹配`, `i 不动`, `回退 j (j = next[j - 1])`, 然后 `接着循环比较`
       - `j` 跳到 `模式串` 中 `最长可复用的匹配前缀长度`

5. 前缀/后缀/真前缀/真后缀
   - 前缀和后缀
     - 前缀
       - 字符串从开头开始的连续子串
       - 如: 字符串 "abcde" 的前缀包括: ""（空串）、"a"、"ab"、"abc"、"abcd"、"abcde"（整个字符串）
     - 后缀
       - 字符串从结尾开始的连续子串
       - 如: 字符串 "abcde" 的后缀包括: ""（空串）、"e"、"de"、"cde"、"bcde"、"abcde"（整个字符串）
   - 真前缀/真后缀
     - 真前缀: 前缀中不包含整个字符串本身, 即，除去整个字符串，剩下的前缀都叫真前缀
       - 对 "abcde"，真前缀是: ""、"a"、"ab"、"abc"、"abcd"
     - 真后缀: 后缀中不包含整个字符串本身
       - 对 "abcde"，真后缀是: ""、"e"、"de"、"cde"、"bcde"

6. 举例
   ```text
   主串(text): ababcabcabababd
   模式串(pattern): ababd
   
   1. 构造模式串的 `next` 数组(部分匹配表)
   next 数组长度为 5
   
   pattern = "a b a b a c a"  -- 长度为 7
   index:     0 1 2 3 4 5 6
   
   1.1 定义辅助指针 `j`
       - `i`: 当前计算的位置(从 `1` 开始)
       - `j`: 当前最长相等前后缀的长度(也是 `next[i-1]`)
       - 初始: next[0] = 0, j = 0
   
   1.2 构造过程
       用循环, 从 `i = 1` 到 `i = 6` 逐步求 `next[i]`
     
   i = 1
   -> 比较 pattern[i] 和 pattern[j], 即 pattern[1] 和 pattern[0]
   -> pattern[1] = b, pattern[0] = a
   -> a != b, 且 j = 0, 没有更短的前后缀可退回
   -> next[1] = 0
   
   i = 2
   -> 比较 pattern[i] 和 pattern[j], 即 pattern[2] 和 pattern[0] 
   -> pattern[2] = a, pattern[0] = a
   -> a = a, 相等, j += 1 => j = 1
   -> next[2] = j = 1
   
   i = 3
   -> 比较 pattern[i] 和 pattern[j], 即 pattern[3] 和 pattern[1]     
   -> pattern[3] = b, pattern[1] = b
   -> b = b, 相等, j += 1 => j = 2
   -> next[3] = j = 2
   
   i = 4
   -> 比较 pattern[i] 和 pattern[j], 即 pattern[4] 和 pattern[2]    
   -> pattern[4] = a, pattern[2] = a
   -> a = a, 相等, j += 1 => j = 3
   -> next[3] = j = 3
   
   i = 5
   -> 比较 pattern[i] 和 pattern[j], 即 pattern[5] 和 pattern[3]    
   -> pattern[5] = c, pattern[3] = b
   -> c != b, 回退: j = next[j - 1] = next[2] = 1
   -> 重新比较 pattern[i] 和 pattern[j], 即 pattern[5] 和 pattern[1]    
   -> pattern[5] = c, pattern[1] = b
   -> c != b, 继续退: j = next[j - 1] = next[0] = 0
   -> 重新比较 pattern[i] 和 pattern[j], 即 pattern[5] 和 pattern[0]    
   -> pattern[5] = c, pattern[0] = a
   -> c != b, 且 j = 0, 没有更短的前后缀可退回
   -> next[5] = 0
   
   i = 6
   -> 比较 pattern[i] 和 pattern[j], 即 pattern[6] 和 pattern[0]   
   -> pattern[6] = a, pattern[0] = a
   -> a = a, 相等, j += 1 => j = 1
   -> next[6] = 1
   
   最终:
   pattern:  a  b  a  b  a  c  a
   index:    0  1  2  3  4  5  6
   next:     0  0  1  2  3  0  1
   
   主串:
   text:  "a b a b a b c a b a  b  a  c  a"
   index:  0 1 2 3 4 5 6 7 8 9 10 11 12 13

   2. 匹配过程
   2.1 两指针, 一个指针 `i` 指向主串, 另一个指针 `j` 指向模式串, 初始为 `0`
  
   i = 0
   -> 比较 text[i] 和 pattern[i], 即 text[0] 和 pattern[0]
   -> text[0] = a, pattern[0] = a
   -> a = a, 相等, i += 1 = 1, j += 1 = 1
   
   i = 1
   -> 比较 text[i] 和 pattern[i], 即 text[1] 和 pattern[1]  
   -> text[1] = b, pattern[1] = b
   -> b = b, 相等, i += 1 = 2, j += 1 = 2
   
   i = 2
   -> 比较 text[i] 和 pattern[i], 即 text[2] 和 pattern[2]  
   -> text[2] = a, pattern[2] = a
   -> a = a, 相等, i += 1 = 3, j += 1 = 3
   
   i = 3
   -> 比较 text[i] 和 pattern[i], 即 text[3] 和 pattern[3]  
   -> text[3] = b, pattern[2] = b
   -> b = b, 相等, i += 1 = 4, j += 1 = 4
   
   i = 4
   -> 比较 text[i] 和 pattern[i], 即 text[4] 和 pattern[4]  
   -> text[4] = b, pattern[4] = b
   -> a = a, 相等, i += 1 = 5, j += 1 = 5
   
   i = 5
   -> 比较 text[i] 和 pattern[i], 即 text[5] 和 pattern[5]  
   -> text[5] = b, pattern[4] = c  
   -> b != c, 回退 j: next[j - 1] = next[4] = 3, i 不动, j = 3
   -> 继续比较 text[i] 和 pattern[i], 即 text[5] 和 pattern[3] 
   -> text[5] = b, pattern[3] = b 
   -> b = b, 相等, i += 1 = 6, j += 1 = 4
   -> 继续比较 text[i] 和 pattern[i], 即 text[6] 和 pattern[4]  
   -> text[6] = c, pattern[4] = a
   -> c != a, 回退j: next[j - 1] = next[3] = 2, i 不动, j = 2
   -> 继续比较 text[i] 和 pattern[i], 即 text[6] 和 pattern[2]
   -> text[6] = c, pattern[2] = a
   -> c != a, 回退j: next[j - 1] = next[1] = 0, i 不动, j = 0
   -> 继续比较 text[i] 和 pattern[i], 即 text[6] 和 pattern[0]  
   -> text[6] = c, pattern[0] = a  
   -> c != a, 此时 j = 0, 无法回退
   -> 主串指针前移, i += 1 = 7, 此时 j = 0
     
   i = 7
   -> 比较 text[i] 和 pattern[i], 即 text[7] 和 pattern[0]  
   -> text[7] = a, pattern[0] = a  
   -> a = a, 相等, i += 1 = 8, j += 1 = 1
   
   i = 8
   -> 比较 text[i] 和 pattern[i], 即 text[8] 和 pattern[1] 
   -> text[8] = b, pattern[1] = b  
   -> b = b, 相等, i += 1 = 9, j += 1 = 2
   
   i = 9
   -> 比较 text[i] 和 pattern[i], 即 text[9] 和 pattern[2]  
   -> text[9] = a, pattern[2] = a
   -> a = a, 相等, i += 1 = 10, j += 1 = 3
   
   i = 10
   -> 比较 text[i] 和 pattern[i], 即 text[10] 和 pattern[3]  
   -> text[10] = b, pattern[3] = b
   -> b = b, 相等, i += 1 = 11, j += 1 = 4
   
   i = 11
   -> 比较 text[i] 和 pattern[i], 即 text[11] 和 pattern[4]  
   -> text[11] = a, pattern[4] = a 
   -> a = a, 相等, i += 1 = 12, j += 1 = 5
   
   i = 12
   -> 比较 text[i] 和 pattern[i], 即 text[12] 和 pattern[5]  
   -> text[12] = c, pattern[5] = c 
   -> c = c, 相等, i += 1 = 13, j += 1 = 6
   
   i = 13
   -> 比较 text[i] 和 pattern[i], 即 text[13] 和 pattern[6]  
   -> text[13] = a, pattern[6] = a 
   -> a = a, 相等, i += 1 = 14, j += 1 = 7
   
   最后:
   j = 7 等于模式串的长度, 匹配成功
   匹配起始位置是 i - j = 14 - 7 = 7
   ```