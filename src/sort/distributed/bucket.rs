/*!
    桶排序(Bucket Sort)
    1. 思想: 将数据分到有限数量的桶中，每个桶内再分别排序
    2. 时间复杂度:
       - 最差：O(n2)(数据分布不均)
       - 平均: O(n + k)
    3. 空间复杂度: O(n + k)
    4. 特点: 适合分布均匀的数据
*/

pub struct Bucket {}