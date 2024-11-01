use bytemuck::{Pod, Zeroable};  // 引入 bytemuck 库，以支持零拷贝数据结构
use steel::*;  // 引入 steel 库，可能用于处理指令和账户元数据

/// 定义 MineEvent 结构体，用于表示挖矿事件的数据。
#[repr(C)]  // 指定为 C 语言兼容的内存布局
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]  // 实现相关 trait
pub struct MineEvent {
    pub difficulty: u64,  // 当前挖矿的难度
    pub reward: u64,      // 挖矿奖励金额
    pub timing: i64,      // 事件的时间戳
}

// 为 MineEvent 结构体生成事件相关的实现。
event!(MineEvent);