use bytemuck::{Pod, Zeroable};  // 引入 bytemuck 库以支持零拷贝数据结构
use solana_program::pubkey::Pubkey;  // 引入 Solana 的 Pubkey 类型
use steel::*;  // 引入 steel 库，可能用于处理指令和账户元数据

use crate::consts::PROOF;  // 引入常量 PROOF

use super::LuckycoinAccount;  // 引入父模块中的 LuckycoinAccount

/// Proof 账户用于追踪矿工的当前哈希、可领取的奖励和一生的统计数据。
/// 每个矿工只能拥有一个 proof 账户，该账户是程序进行挖矿或领取奖励所必需的。
#[repr(C)]  // 指定为 C 语言兼容的内存布局
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]  // 实现相关 trait
pub struct Proof {
    /// 授权使用此 proof 账户的签名者。
    pub authority: Pubkey,

    /// 矿工已质押或获得的代币数量。
    pub balance: u64,

    /// 当前的挖矿挑战。
    pub challenge: [u8; 32],

    /// 矿工提供的最后一个哈希。
    pub last_hash: [u8; 32],

    /// 这个账户最后一次提供哈希的时间戳。
    pub last_hash_at: i64,

    /// 这个账户最后一次存入质押的时间戳。
    pub last_stake_at: i64,

    /// 允许提交挖矿哈希的密钥对。
    pub miner: Pubkey,

    /// 该矿工提供的总哈希数量。
    pub total_hashes: u64,

    /// 分配给该矿工的总奖励数量。
    pub total_rewards: u64,
}

/// 计算给定矿工的 proof 账户的程序派生地址 (PDA)。
///
/// # 参数
/// - `authority`: 矿工的公钥，用于生成唯一的 PDA。
///
/// # 返回
/// 返回生成的 PDA 和一个种子字节。
pub fn proof_pda(authority: Pubkey) -> (Pubkey, u8) {
    // 根据给定的 authority 和程序 ID 计算 PDA
    Pubkey::find_program_address(&[PROOF, authority.as_ref()], &crate::id())
}

// 为 LuckycoinAccount 生成与 Proof 结构体相关的账户实现。
account!(LuckycoinAccount, Proof);