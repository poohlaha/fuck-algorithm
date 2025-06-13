/*!
  账户 + 合约调用(Contract Call) + 交易类型
  账户: 地址、余额，可更新余额
  合约调用(Contract Call): 区块链系统中与 `智能合约交互` 的核心机制，是与 `链上程序`打交道的方式
  可以把智能合约看成是 `部署在区块链上的自动化程序或服务`，它们能 `存储状态`、`执行逻辑`, 调用合约，就像在区块链上调用一个函数，常见的形式包括：
    - 修改状态（例如发币、投票）
    - 查询信息（例如查看用户余额）
    - 调用复杂逻辑（例如 DEX 兑换、NFT 转让）
*/

// 账户
#[derive(Debug)]
pub struct Account {
    pub balance: i64, // 余额
}

impl Account {
    pub fn new(balance: i64) -> Self {
        Self { balance }
    }
}

// 合约调用
pub enum ContractResult {
    ReturnValue(String),
    Ok,
}

// 交易类型
#[derive(Debug, Clone)]
pub enum TransactionType {
    Transfer {
        from: String,
        to: String,
        amount: i64,
    },
    ContractCall {
        caller: String,
        contract: String,
        method: String,
        args: Vec<String>,
    },
}
