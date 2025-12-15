/*!
   1233. 删除子文件夹
   力扣: https://leetcode.cn/problems/remove-sub-folders-from-the-filesystem/description
   题目: 你是一位系统管理员，手里有一份文件夹列表 folder，你的任务是要删除该列表中的所有 子文件夹，并以 任意顺序 返回剩下的文件夹。
        如果文件夹 folder[i] 位于另一个文件夹 folder[j] 下，那么 folder[i] 就是 folder[j] 的 子文件夹 。folder[j] 的子文件夹必须以 folder[j] 开头，后跟一个 "/"。例如，"/a/b" 是 "/a" 的一个子文件夹，但 "/b" 不是 "/a/b/c" 的一个子文件夹。
        文件夹的「路径」是由一个或多个按以下格式串联形成的字符串：'/' 后跟一个或者多个小写英文字母。
        例如，"/leetcode" 和 "/leetcode/problems" 都是有效的路径，而空字符串和 "/" 不是。

  示例:
      1. 输入: folder = ["/a","/a/b","/c/d","/c/d/e","/c/f"]
         输出: ["/a","/c/d","/c/f"]
         解释: "/a/b" 是 "/a" 的子文件夹，而 "/c/d/e" 是 "/c/d" 的子文件夹。

      2. 输入: folder = ["/a","/a/b/c","/a/b/d"]
         输出: ["/a"]

      3. 输入: folder = ["/a/b/c","/a/b/ca","/a/b/d"]
         输出: ["/a/b/c","/a/b/ca","/a/b/d"]

   解: 排序 + 单指针
       原始: ["/a","/a/b","/c/d","/c/d/e","/c/f"]
       排序: ["/a","/a/b","/c/d","/c/d/e","/c/f"]
       发现:
             /a 后面紧跟着它的所有子文件夹
             /c/d 后面紧跟着 /c/d/e
       不会出现 `子文件夹跑到父文件夹前面`, 因此: 只要维护一个 `上一个保留的文件夹`
*/
pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    if folder.is_empty() {
        return Vec::new();
    }

    let mut folder = folder.clone();
    folder.sort();

    let mut result: Vec<String> = Vec::new();

    for f in folder {
        if let Some(prev) = result.last() {
            let prefix = format!("{}/", prev);
            if f.starts_with(&prefix) {
                continue; // 子文件夹，跳过
            }
        }

        result.push(f.clone());
    }

    result
}
