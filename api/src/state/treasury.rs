use bytemuck::{Pod, Zeroable};  // 引入 bytemuck 库以支持零拷贝数据结构
use solana_program::pubkey::Pubkey;  // 引入 Solana 的 Pubkey 类型
use steel::*;  // 引入 steel 库，可能用于处理指令和账户元数据

use crate::consts::TREASURY;  // 引入常量 TREASURY

use super::LuckycoinAccount;  // 引入父模块中的 LuckycoinAccount

/// Treasury 是一个单例账户，作为 ORE 代币的铸币权限和程序全局代币账户的权限。
#[repr(C)]  // 指定为 C 语言兼容的内存布局
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]  // 实现相关 trait
pub struct Treasury {}

/// 计算财政账户的程序派生地址 (PDA)。
pub fn treasury_pda() -> (Pubkey, u8) {
    // 使用 TREASURY 和程序 ID 计算 PDA
    Pubkey::find_program_address(&[TREASURY], &crate::id())
}

// 为 LuckycoinAccount 生成与 Treasury 结构体相关的账户实现。
account!(LuckycoinAccount, Treasury);