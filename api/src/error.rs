use num_enum::IntoPrimitive;  // 引入 IntoPrimitive trait，用于将枚举转换为原始整数
use steel::*;  // 引入 steel 库，可能用于处理指令和账户元数据
use thiserror::Error;  // 引入 thiserror 库，以便简化错误处理

/// 定义 LuckycoinError 枚举，用于表示程序中的各种错误情况。
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]  // 为枚举实现相关 trait
#[repr(u32)]  // 指定枚举的底层表示为无符号 32 位整数
pub enum LuckycoinError {
    #[error("The epoch has ended and needs reset")]
    NeedsReset = 0,  // 周期已结束，需要重置

    #[error("The provided hash is invalid")]
    HashInvalid = 1,  // 提供的哈希无效

    #[error("The provided hash did not satisfy the minimum required difficulty")]
    HashTooEasy = 2,  // 提供的哈希未达到最低要求的难度

    #[error("The claim amount cannot be greater than the claimable rewards")]
    ClaimTooLarge = 3,  // 领取金额不能大于可领取的奖励

    #[error("The clock time is invalid")]
    ClockInvalid = 4,  // 时钟时间无效

    #[error("You are trying to submit too soon")]
    Spam = 5,  // 提交过于频繁

    #[error("The maximum supply has been reached")]
    MaxSupply = 6,  // 已达到最大供应量

    #[error("The proof does not match the expected account")]
    AuthFailed = 7,  // 证明与预期账户不匹配
}

// 为 LuckycoinError 枚举生成错误处理相关的实现。
error!(LuckycoinError);
