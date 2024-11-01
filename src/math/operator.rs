/**!
 1. 按位与（AND）&
   描述：对两个整数的每一位进行与操作，只有在两个对应的位都是1时结果才为1
   示例:
       let a = 5; // 二进制 0101
       let b = 3; // 二进制 0011
       let result = a & b; // 结果是 1 (0001)
       println!("{}", result); // 输出: 1

  2. 按位或（OR）|
    描述：对两个整数的每一位进行或操作，只要有一个对应的位是1，结果就是1。
    示例:
       let a = 5; // 二进制 0101
       let b = 3; // 二进制 0011
       let result = a | b; // 结果是 7 (0111)
       println!("{}", result); // 输出: 7

   3. 按位异或（XOR）^
     描述：对两个整数的每一位进行异或操作，只有在两个对应的位不同的时候结果才为1。
     示例:
         let a = 5; // 二进制 0101
         let b = 3; // 二进制 0011
         let result = a ^ b; // 结果是 6 (0110)
         println!("{}", result); // 输出: 6

   4. 按位取反（NOT）!
     描述：对一个整数的每一位进行取反操作，0变1，1变0。
     示例:
         let a = 5; // 二进制 0101
         let result = !a; // 结果是 -6 (取反后为 1010，若按补码表示)
         println!("{}", result); // 输出: -6

    5. 左移（Left Shift）<<
     描述：将一个整数的二进制位向左移动指定的位数，右边补0。
     示例:
          let a = 5; // 二进制 0101
          let result = a << 1; // 结果是 10 (1010)
          println!("{}", result); // 输出: 10

     6. 右移（Right Shift）>>
       描述：将一个整数的二进制位向右移动指定的位数，左边的补位取决于整数的符号（符号位移或逻辑位移）。
       示例:
           let a = 5; // 二进制 0101
           let result = a >> 1; // 结果是 2 (0010)
           println!("{}", result); // 输出: 2

     7. 无符号右移（Logical Right Shift）
        描述: 在某些语言中，存在无符号右移操作符（例如 >>>），在 Rust 中可以使用 >> 操作符配合类型转换。
        示例:
           let a: u32 = 0b11111111_11111111_11111111_11111111; // 大于 32 位
           let result = a >> 1; // 结果是 0b01111111_11111111_11111111_11111111
           println!("{}", result); // 输出: 2147483647


    其他用途:
      1. 利用或操作 `|` 转换为小写
        操作说明：使用位或操作符 | 将字符与空格 ' ' 进行操作。
        ASCII 值：在 ASCII 编码中，英文字母小写和大写的差别在于二进制的某些位（具体是第六位）。大写字母的 ASCII 值比相应的小写字母小 `32`(空格的ASCII为32（00100000）)。
        示例:
           ('A' | ' ')  // A的ASCII为65（01000001），空格的ASCII为32（00100000），结果是 97（01100001）即 'a'
           ('a' | ' ')  // a的ASCII为97（01100001），结果仍然是 97（01100001）即 'a'

      2. 利用与操作 `&` 转换为大写
        操作说明：使用位与操作符 & 将字符与下划线 '_' 进行操作。
        ASCII 值：与空格相似，大写字母的 ASCII 值在第六位是 0，而小写字母在第六位是 1。通过与下划线的位与操作可以将小写字母转换为大写字母, 下划线的ASCII为95（01011111）。
        示例:
           ('b' & '_')  // b的ASCII为98（01100010），下划线的ASCII为95（01011111），结果是 66（01000010）即 'B'
           ('B' & '_')  // B的ASCII为66（01000010），结果仍然是 66（01000010）即 'B'

      3. 利用异或操作 `^` 进行大小写互换
        操作说明：使用位异或操作符 ^ 将字符与空格 ' ' 进行操作。。
        ASCII 值：异或操作可以用于切换字符的大小写，因为同一字符与空格的异或结果会在二进制的第六位上切换（0 ↔ 1）。
        示例:
           ('d' ^ ' ')  // d的ASCII为100（01100100），空格的ASCII为32（00100000），结果是 68（01000100）即 'D'
           ('D' ^ ' ')  // D的ASCII为68（01000100），结果是 100（01100100）即 'd'

      总结:
       位或 |：用于将字符转换为小写（字符与空格的或操作）。
       位与 &：用于将字符转换为大写（字符与下划线的与操作）。
       位异或 ^：用于实现字符大小写的互换（字符与空格的异或操作）。

     4. 使用异或操作符交换两个数
       a ^= b;  // Step 1: a = a ^ b
       b ^= a;  // Step 2: b = b ^ (a ^ b) => b = a
       a ^= b;  // Step 3: a = (a ^ b) ^ b => a = b

    5. 加一
       描述: 按位取反 ~n: 按位取反操作符 ~ 会将整数 n 的每个二进制位反转。比如，如果 n 是 1（二进制为 0000...0001），则 ~n 将变为 -2（在二进制中为 1111...1110，取决于系统的位数）
       int n = 1;
       n = -~n; // rust 中 ～ 改成 !

    6. 减一
       int n = 2;
       n = ~-n; // rust 中 ～ 改成 !

    7. 判断两个数是否异号
       int x = -1, y = 2;
       boolean f = ((x ^ y) < 0); // true

       int x = 3, y = 2;
       boolean f = ((x ^ y) < 0); // false

    8. index & (arr.length - 1)
      主要用于实现循环数组（或环形缓冲区）的索引计算。它的主要作用是确保索引在数组的有效范围内，同时利用位运算的高效性。
     - index & (arr.length - 1) 的运算将 index 的二进制表示的高位部分清零，只保留低位，确保得到的结果不会超出数组的有效范围。例如，假设数组的长度为 8，可能的索引值是 0 到 7。
     - 如果 index 超过 7，使用 index & 7 进行运算可以将其“折叠”到有效范围内。
     注: 这个技巧只适用于数组长度是 2 的幂次方的情况。

    9. n & (n-1) 的运用
     广泛用于处理与二进制相关的问题，尤其是检测一个数是否是 2 的幂、计算二进制中 1 的个数等。以下是这个运算的解释和应用示例。
     运算原理：
       - n 是一个正整数，如果 n 是 2 的幂，那么它的二进制表示中只有一个 1，其余全是 0。例如，4 的二进制是 100，8 是 1000。
       - n - 1 会将 n 的二进制表示中最右边的 1 变为 0，其余位变为 1。例如，4 的 n - 1 是 3（即 011）。
       - 进行与操作 n & (n - 1) 时，只有当 n 是 2 的幂时，结果才会是 0。
    示例: 检测一个数是否是 2 的幂：n != 0 && (n & (n - 1)) == 0

    10. a ^ a = 0 的运用
        一个数和它本身做异或运算结果为 0，即 a ^ a = 0；
        一个数和 0 做异或运算的结果为它本身，即 a ^ 0 = a。
       用途:
        - 找到数组中唯一出现的元素(所有成对元素会相互抵消)
        - 交换两个变量的值（不使用临时变量）, 交换三次
           x = x ^ y;
           y = x ^ y;
           x = x ^ y;
        - 检测数组是否包含重复元素


*/
pub(crate) fn operator() {
    // 按位与（AND）&
    let a = 5; // 二进制 0101
    let b = 3; // 二进制 0011
    let result = a & b; // 结果是 1 (0001)
    println!("按位与（AND）&: {}", result); // 输出: 1

    // 按位或（OR）|
    let a = 5; // 二进制 0101
    let b = 3; // 二进制 0011
    let result = a | b; // 结果是 7 (0111)
    println!("按位或（OR）|: {}", result); // 输出: 7

    // 按位异或（XOR）^
    let a = 5; // 二进制 0101
    let b = 3; // 二进制 0011
    let result = a ^ b; // 结果是 6 (0110)
    println!("按位异或（XOR）^: {}", result); // 输出: 6

    // 按位取反（NOT）!
    let a = 5; // 二进制 0101
    let result = !a; // 结果是 -6 (取反后为 1010，若按补码表示)
    println!("按位取反（NOT）!: {}", result); // 输出: -6

    // 左移（Left Shift）<<
    let a = 5; // 二进制 0101
    let result = a >> 1; // 结果是 2 (0010)
    println!("左移（Left Shift）<<: {}", result); // 输出: 2

    // 使用异或操作 ^ 来进行字符的大小写互换, 将 char 转换为其对应的 ASCII 值（即 u8 类型）
    let lower = 'd';
    let upper = (lower as u8) ^ b' '; // 将小写转换为大写
    println!("异或操作 ^ 转大写: {}", upper as char); // 输出: D

    let back_to_lower = (upper as u8) ^ b' '; // 将大写转换为小写
    println!("异或操作 ^ 转小写: {}", back_to_lower as char); // 输出: d

    // 全转大写 或小写
    fn to_uppercase(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_ascii_lowercase() {
                    // 将小写字母转换为大写
                    (c as u8 ^ b' ') as char
                } else {
                    c // 直接返回非小写字母
                }
            })
            .collect()
    }

    fn to_lowercase(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_ascii_uppercase() {
                    // 将大写字母转换为小写
                    (c as u8 ^ b' ') as char
                } else {
                    c // 直接返回非大写字母
                }
            })
            .collect()
    }

    let original = "Hello World!";
    let uppercased = to_uppercase(original);
    let lowercased = to_lowercase(original);
    println!("Original: {}", original); // 输出: Hello World!
    println!("Uppercased: {}", uppercased); // 输出: HELLO WORLD!
    println!("Lowercased: {}", lowercased); // 输出: hello world!

    // 计算二进制中 1 的个数
    fn count_ones(n: u32) -> u32 {
        let mut count = 0;
        let mut num = n;

        while num > 0 {
            count += 1;
            num &= num - 1; // 每次将最低位的 1 置为 0
        }

        count
    }

    let num = 29; // 29 的二进制是 11101
    println!("{} 的二进制中 1 的个数是 {}", num, count_ones(num)); // 输出: 4
}

/**
    位1的个数(计算汉明权重)
    力扣: https://leetcode.cn/problems/number-of-1-bits/description/
    题目: 给定一个正整数 n，编写一个函数，获取一个正整数的二进制形式并返回其二进制表达式中 `设置位`(set bit 指在某数的二进制表示中值为 1 的二进制位) 的个数（也被称为汉明重量）。
    答:
*/
pub(crate) fn hamming_weight(n: usize) -> u32 {
    let mut count: u32 = 0;

    let mut num = n;
    while num > 0 {
        count += 1;
        num &= num - 1
    }

    return count;
}

/**
  2 的幂
  力扣: https://leetcode.cn/problems/power-of-two/description/
  题目: 给你一个整数 n，请你判断该整数是否是 2 的幂次方。如果是，返回 true ；否则，返回 false 。
       如果存在一个整数 x 使得 n == 2x ，则认为 n 是 2 的幂次方。
*/
pub(crate) fn is_power_of_two(n: usize) -> bool {
    return n != 0 && (n & (n - 1)) == 0;
}

/**
  只出现一次的数字
  力扣: https://leetcode.cn/problems/single-number/description/
  题目: 给你一个 非空 整数数组 nums ，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。
       你必须设计并实现线性时间复杂度的算法来解决此问题，且该算法只使用常量额外空间。
  解:
     1. 任何数与 0 异或都保持不变：a ^ 0 = a
     2. 任何数与自身异或都变为 0：a ^ a = 0
     3. 异或运算满足交换律和结合律，即顺序可以不影响结果。
     假设数组 nums 中，所有元素都出现了两次，只有一个元素出现一次。异或操作的特性保证了，如果我们对数组中的每个元素逐个异或，最终的结果将是那个只出现了一次的元素，因为相同的元素会抵消成 0。
  重点: 成对元素相互抵消

  如果检测数组是否包含重复元素, 则 只要 != 0 就行
*/
pub fn single_number(nums: Vec<i32>) -> i32 {
    let unique = nums.iter().fold(0, |acc, &num| acc ^ num);

    // [4, 1, 2, 1, 2] -> 4
    println!("唯一出现的元素是: {}", unique);
    unique
}

/**
  丢失的数字
  力扣: https://leetcode.cn/problems/missing-number/description/
  题目: 给定一个包含 [0, n] 中 n 个数的数组 nums ，找出 [0, n] 这个范围内没有出现在数组中的那个数。
  解: 也可以理解为: 有个等差数列 0, 1, 2,..., n，其中少了某一个数字，请你把它找出来, sum(0,1,..n) - sum(nums)
     这里使用 a ^ 0 = a, a ^ a = 0
     index:  0 1 2 3 4
     nums:   0 1   3 4
     索引和数字成对出现, 可以抵消
*/
pub fn missing_number(nums: Vec<i32>) -> i32 {

    // 索引和数字成对出现, 可以抵消
    fn missing1(nums: Vec<i32>) -> i32 {
        let mut res = nums.len() as i32; // 初始化为 n
        for (i, num) in nums.iter().enumerate() {
            res ^= i as i32 ^ num; // 索引和数组中数字异或
        }
        res
    }

    // 等差数列: 0, 1, 2,..., n, [0, n] 的所有数字之和是 sum = n * (n + 1) / 2，则数组的实际和与预期和之间的差就是缺失的数字。
    fn missing2(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let expected_sum = n * (n + 1) / 2; // 预期和
        let actual_sum: i32 = nums.iter().sum(); // 数组中实际和
        expected_sum - actual_sum // 差值即为缺失的数字
    }

    missing1(nums)
}
