/*!
   22. 括号生成
   力扣: https://leetcode.cn/problems/generate-parentheses/description/
   题目: 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
   示例:
      输入: n = 3
      输出: ["((()))","(()())","(())()","()(())","()()()"]

      输入: n = 1
      输出: ["()"]

   解: 回溯算法
       时间复杂度: O(4^n / √n)
       空间复杂度: O(C_n × n)
*/
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    if n == 0 {
        return Vec::new();
    }

    if n == 1 {
        return vec![String::from("()")];
    }

    fn dfs(n: i32, open: i32, close: i32, cur: &mut String, out: &mut Vec<String>) {
        if open == n && close == n {
            out.push(cur.clone());
            return;
        }

        if open < n {
            cur.push('(');
            dfs(n, open + 1, close, cur, out);
            cur.pop();
        }

        if close < open {
            cur.push(')');
            dfs(n, open, close + 1, cur, out);
            cur.pop();
        }
    }

    let mut out = Vec::new();
    let mut cur = String::new();
    dfs(n, 0, 0, &mut cur, &mut out);
    out
}
