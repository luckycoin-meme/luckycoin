use bytemuck::{Pod, Zeroable};  // 引入 bytemuck 库以支持零拷贝数据结构
use solana_program::pubkey::Pubkey;  // 引入 Solana 的 Pubkey 类型
use steel::*;  // 引入 steel 库，可能用于处理指令和账户元数据

use crate::consts::BUS;  // 引入常量 BUS

use super::LuckycoinAccount;  // 引入父模块中的 OreAccount

/// Bus 账户负责分配挖矿奖励。总共有 8 个 bus 账户，
/// 以最小化写锁争用并允许 Solana 并行处理挖矿指令。
#[repr(C)]  // 指定为 C 语言兼容的内存布局
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]  // 实现相关 trait
pub struct Bus {
    /// bus 账户的 ID。
    pub id: u64,

    /// 当前周期内该 bus 剩余的奖励总额。
    pub rewards: u64,

    /// 如果没有限制，该 bus 在当前周期应支付的奖励总额。
    /// 这用于计算更新后的奖励率。
    pub theoretical_rewards: u64,

    /// 在当前周期内，bus 观察到的最大质押余额。
    pub top_balance: u64,
}

/// 获取 bus 账户的程序派生地址 (PDA)。
pub fn bus_pda(id: u8) -> (Pubkey, u8) {
    // 根据给定的 ID 和程序 ID 计算 PDA
    Pubkey::find_program_address(&[BUS, &[id]], &crate::id())
}

// 为 LuckycoinAccount 生成与 Bus 结构体相关的账户实现。
account!(LuckycoinAccount, Bus);