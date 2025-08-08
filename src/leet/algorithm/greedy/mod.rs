/*!
 [贪心算法](src/algorithm/greedy/greedy.md)
*/

pub mod test;

use std::collections::HashMap;

/**
   12. 整数转罗马数字
   力扣: https://leetcode.cn/problems/integer-to-roman/description/
   题目:
       七个不同的符号代表罗马数字，其值如下:
       符号      值
        I	     1
        V	     5
        X	     10
        L	     50
        C	     100
        D	     500
        M	     1000

       罗马数字是通过添加从最高到最低的小数位值的转换而形成的。将小数位值转换为罗马数字有以下规则:
       - 如果该值不是以 4 或 9 开头，请选择可以从输入中减去的最大值的符号，将该符号附加到结果，减去其值，然后将其余部分转换为罗马数字。
       - 如果该值以 4 或 9 开头，使用 减法形式，表示从以下符号中减去一个符号，例如 4 是 5 (V) 减 1 (I): IV ，9 是 10 (X) 减 1 (I)：IX。
         仅使用以下减法形式：4 (IV)，9 (IX)，40 (XL)，90 (XC)，400 (CD) 和 900 (CM)。
       - 只有 10 的次方（I, X, C, M）最多可以连续附加 3 次以代表 10 的倍数。
         你不能多次附加 5 (V)，50 (L) 或 500 (D)。
         如果需要将符号附加4次，请使用 减法形式。

       给定一个整数，将其转换为罗马数字。
       提示: 1 <= num <= 3999

   总结:
       1. 连续使用规则
          10 的次方(I, X, C, M)
          a. 可以连续出现最多 3 次(可以不考虑, 因为 1 <= num <= 3999):
          - III = 3
          - XXX = 30
          - CCC = 300
          - MMM = 3000

          b. 如果需要 4 次:
          不能写作 IIII，而是使用减法表示法:
          - 4 = IV（5 - 1）
          - 9 = IX（10 - 1）
          - 40 = XL（50 - 10）
          - 90 = XC（100 - 10）
          - 400 = CD（500 - 100）
          - 900 = CM（1000 - 100）

          c. 5 的倍数(V, L, D)
          不能连续重复使用:
          - 不允许写 VV (10)，应写作 X
          - 不允许写 LL (100)，应写作 C
          - 不允许写 DD (1000)，应写作 M

       2. 减法表示法
          在以下情况下使用:
          4 和 9 的场景需要用“减法形式”:
          - 4 = IV（5 - 1）
          - 9 = IX（10 - 1）
          - 40 = XL（50 - 10）
          - 90 = XC（100 - 10）
          - 400 = CD（500 - 100）
          - 900 = CM（1000 - 100）
          除此之外，其他数字直接用最大可减去的值组合即可

        3. 转换过程(分解型直觉)
           给定整数 num，循环:
           如果遇到以 4 或 9 开头（如 400, 90, 9 等），使用减法形式（CM, XC, IX 等），并减去其值
           否则:
              - 找小于等于当前 num 的 最大符号值（如 1000, 500, 100 等）
              - 附加对应符号
              - 从 num 中减去对应值
           重复直到 num == 0。

       4. 示例
          num = 1994
          最大符号 ≤ 1994 → 1000(M)
          -> res = M
          -> num = 994

          994 >= 900(CM)
          -> res = MCM
          -> num = 94

          94 >= 90(XC)
          -> res = MCMXC
          -> num = 4

          4 = 4(IV)
          -> res = MCMXCIV
          -> num = 0
      口诀: 能减则减（4/9 时），不能减则取最大符号减之，重复符号最多 3 次，VLD 不可连用

   解:
     使用贪心算法
     - 罗马数字系统是严格的，当前值总能被唯一拆分为若干合法的符号组合(无回溯)
     - 为了保证符号尽可能少、保证符号合法
       每次都用最大的可减符号(M, CM, D, CD, C, XC, L, XL, X, IX, V, IV, I)

    符号表:
    1000 -> M
    900 -> CM(1000 - 100)
    500 -> D
    400 -> CD(500 - 100)
    100 -> C
     90 -> XC(100 - 10)
     50 -> L
     40 -> XL(50 - 10)
     10 -> X
      9 -> IX(10 - 1)
      5 -> V
      4 -> IV(5 - 1)
      1 -> I
*/
pub fn int_to_roman(num: i32) -> String {
    // 设置符号表
    let symbols = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut res = String::new();

    let mut temp = num;
    for &(k, v) in symbols.iter() {
        while temp >= k {
            temp -= k;
            res.push_str(v);
        }
    }

    res
}

/**
   13. 罗马数字转整数
   力扣: https://leetcode.cn/problems/roman-to-integer/description/
   题目: 罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
        字符      值
         I	     1
         V	     5
         X	     10
         L	     50
         C	     100
         D	     500
         M	     1000
       例如， 罗马数字 2 写做 II ，即为两个并列的 1 。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II
       通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。
       数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。
       同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况:
       - I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
       - X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
       - C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。

       给定一个罗马数字，将其转换成整数。
*/
pub fn roman_to_int(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut map = HashMap::new();
    map.insert("M", 1000);
    map.insert("CM", 900);
    map.insert("D", 500);
    map.insert("CD", 400);
    map.insert("C", 100);
    map.insert("XC", 90);
    map.insert("L", 50);
    map.insert("XL", 40);
    map.insert("X", 10);
    map.insert("IX", 9);
    map.insert("V", 5);
    map.insert("IV", 4);
    map.insert("I", 1);

    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    let (mut left, mut right) = (0, 0);
    let mut num = 0;

    while right < n {
        right += 1;

        let index = right + 1;
        if index <= n {
            let str = &s[left..index];
            if map.contains_key(str) {
                num += map[str];
                right = index;
                left = right;
                continue;
            }
        }

        let str = &s[left..right];
        if map.contains_key(str) {
            num += map[str];
        }

        left += 1;
    }

    num
}
