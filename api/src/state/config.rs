use bytemuck::{Pod, Zeroable};  // 引入 bytemuck 库以支持零拷贝数据结构
use solana_program::pubkey::Pubkey;  // 引入 Solana 的 Pubkey 类型
use steel::*;  // 引入 steel 库，可能用于处理指令和账户元数据

use crate::consts::CONFIG;  // 引入常量 CONFIG

use super::LuckycoinAccount;  // 引入父模块中的 LuckycoinAccount

/// Config 是一个单例账户，用于管理程序的全局变量。
#[repr(C)]  // 指定为 C 语言兼容的内存布局
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]  // 实现相关 trait
pub struct Config {
    /// 针对最低难度的哈希支付的基础奖励率。
    pub base_reward_rate: u64,

    /// 上一次重置的时间戳。
    pub last_reset_at: i64,

    /// 最低接受的难度。
    pub min_difficulty: u64,

    /// 上一个周期内网络上观察到的最大质押余额。
    pub top_balance: u64,
}

/// 计算配置账户的程序派生地址 (PDA)。
pub fn config_pda() -> (Pubkey, u8) {
    // 根据 CONFIG 和程序 ID 计算 PDA
    Pubkey::find_program_address(&[CONFIG], &crate::id())
}

// 为 LuckycoinAccount 生成与 Config 结构体相关的账户实现。
account!(LuckycoinAccount, Config);