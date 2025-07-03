pub mod test;

pub struct Stack;

impl Stack {
    /**
      20. 有效的括号
      地址: https://leetcode.cn/problems/valid-parentheses/description/
      题目: 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效
           有效字符串需满足:
           1. 左括号必须用相同类型的右括号闭合。
           2. 左括号必须以正确的顺序闭合。
           3. 每个右括号都有一个对应的相同类型的左括号。
    */
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() {
            return false;
        }

        if s.len() == 1 {
            return false;
        }

        let chars = s.chars().collect::<Vec<char>>();
        let mut stack: Vec<char> = Vec::new();
        for c in chars {
            if c == '(' || c == '{' || c == '[' {
                stack.push(c);
                continue;
            }

            if c == ')' || c == '}' || c == ']' {
                // 检查栈顶
                if let Some(&val) = stack.last() {
                    if (c == ')' && val == '(')
                        || (c == ']' && val == '[')
                        || (c == '}' && val == '{')
                    {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        stack.is_empty()
    }

    /**
      42. 接雨水
      题目: 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水

      解: 用单调递减栈(存放索引), 栈顶为左边界, 当前元素为右边界
         积水高度:  min(左边界高度, 右边界高度) - 底部高度(弹出元素高度)
         宽度: 右边界索引 - 左边界索引 - 1
         面积: 高度 * 宽度
         栈为空, 无法形成积水
     左边界: 阻止水从左边流出去
     右边界: 阻止水从右边流出去
     1. 左边界是阻止水从左边流出去，当右边大于左边时，墙比较高，可以放水了。
     2. 如果两个墙连在一起(弹出后栈为空), 不能放水。
     3. 从左往右遍历, 当遇到当前柱子高度 > 栈顶柱子高度时触发(右边界固定(当前柱子)，左边界向左找(栈顶不断弹出), 只对 `比右边界低的部分` 放水(即弹出部分是比右边界矮的柱子时计算水量))
     4. 直到放空水，然后加入右边元素
     注: 栈顶一直在左边，当前元素是右边，弹出的元素是底部
         左边界: stack.last()
         弹出元素: stack.pop()
     栈(索引): [1, 2, 3], 遇到右边界 4, 开始放水
     -> 弹出 3, 此时栈: [1, 2], 栈顶为 2, bottom = 3
     -> 放 2 ~ 4 之间的水(用 bottom=3 计算)
     -> 弹出 2, 此时栈: [1], 栈顶 1, bottom = 2
     -> 放 1 ~ 4 之间的水(用 bottom=2 计算)
     -> 弹出1, 此时栈为空, 无法放水
    */
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() || height.len() == 1 {
            return 0;
        }

        // 单调递减栈(存放索引), 从栈底到栈顶高度递减
        // 栈底（Vec[0]）是最左边遇到的高柱（通常较高）
        // 栈顶（Vec.last()）是最近遇到的较矮柱（通常较矮）
        let mut stack: Vec<usize> = Vec::new();
        let mut total = 0;

        for i in 0..height.len() {
            if stack.is_empty() {
                stack.push(i);
                continue;
            }

            while let Some(&top) = stack.last() {
                if height[top] < height[i] {
                    let mut bottom = 0;
                    let front = stack.pop();
                    if let Some(front) = front {
                        bottom = front;
                    }

                    if stack.is_empty() {
                        break;
                    }

                    let mut left = 0;
                    let last = stack.last();
                    if let Some(&last) = last {
                        left = last;
                    }

                    let height = std::cmp::min(height[left], height[i]) - height[bottom];
                    let width = (i - left - 1) as i32;
                    total += width * height;
                } else {
                    break;
                }
            }

            stack.push(i);
        }

        total
    }
}
