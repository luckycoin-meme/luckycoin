use bytemuck::{Pod, Zeroable};  // 引入 bytemuck 库以支持零拷贝数据结构
use num_enum::TryFromPrimitive;  // 引入 num_enum 库以支持从原始类型转换
use steel::*;  // 引入 steel 库，可能用于处理指令和账户元数据

/// 定义 Luckycoin 指令的枚举类型。
#[repr(u8)]  // 指定此枚举的底层表示为无符号 8 位整数
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]  // 实现 trait
pub enum LuckycoinInstruction {
    Claim = 0,    
    Close = 1,    
    Mine = 2,    
    Open = 3,     
    Reset = 4,    
    Stake = 5,   
    Update = 6, 
    Upgrade = 7, 
    Health = 8, 
    Initialize = 100, 
}

/// 领取指令的结构体。
#[repr(C)]  // 指定内存布局为 C 语言兼容
#[derive(Clone, Copy, Debug, Pod, Zeroable)]  // 实现相关 trait
pub struct Claim {
    pub amount: [u8; 8],  // 领取的金额，使用 8 字节数组表示
}

/// 关闭指令的结构体，未包含额外字段。
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Close {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Health {}

/// 挖矿指令的结构体。
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Mine {
    pub digest: [u8; 16],  // 挖矿结果的摘要，使用 16 字节数组表示
    pub nonce: [u8; 8],    // 用于挖矿的随机数，使用 8 字节数组表示
}

/// 打开账户指令的结构体。
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Open {
    pub bump: u8,  // 用于程序派生地址的 bump 值
}

/// 重置指令的结构体，未包含额外字段。
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Reset {}

/// 质押指令的结构体。
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Stake {
    pub amount: [u8; 8],  // 质押的金额，使用 8 字节数组表示
}

/// 更新指令的结构体，未包含额外字段。
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Update {}

/// 升级指令的结构体。
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Upgrade {
    pub amount: [u8; 8],  // 升级的金额，使用 8 字节数组表示
}

/// 初始化指令的结构体，包含多个 bump 值。
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {
    pub bus_0_bump: u8,  // bus 0 的 bump 值
    pub bus_1_bump: u8,  // bus 1 的 bump 值
    pub bus_2_bump: u8,  // bus 2 的 bump 值
    pub bus_3_bump: u8,  // bus 3 的 bump 值
    pub bus_4_bump: u8,  // bus 4 的 bump 值
    pub bus_5_bump: u8,  // bus 5 的 bump 值
    pub bus_6_bump: u8,  // bus 6 的 bump 值
    pub bus_7_bump: u8,  // bus 7 的 bump 值
    pub config_bump: u8,  // 配置的 bump 值
    pub metadata_bump: u8, // 元数据的 bump 值
    pub mint_bump: u8,    // 铸币的 bump 值
    pub treasury_bump: u8, // 财库的 bump 值
}

// 为每个指令类型生成指令相关的实现。
instruction!(LuckycoinInstruction, Claim);
instruction!(LuckycoinInstruction, Close);
instruction!(LuckycoinInstruction, Mine);
instruction!(LuckycoinInstruction, Open);
instruction!(LuckycoinInstruction, Reset);
instruction!(LuckycoinInstruction, Stake);
instruction!(LuckycoinInstruction, Update);
instruction!(LuckycoinInstruction, Upgrade);
instruction!(LuckycoinInstruction, Health);
instruction!(LuckycoinInstruction, Initialize);