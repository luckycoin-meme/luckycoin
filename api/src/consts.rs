use array_const_fn_init::array_const_fn_init;  // 引入库以初始化常量数组
use const_crypto::ed25519;  // 引入 ed25519 加密库
use solana_program::{pubkey, pubkey::Pubkey};  // 引入 Solana 的 Pubkey 类型

/// 允许初始化程序的权限地址。
pub const INITIALIZER_ADDRESS: Pubkey = pubkey!("DKQEpKgGjNrLH7oF6qF6RQNdQi3nmvEbiAwEVEtAvKsd");

/// 程序初始化时的基础奖励率。
pub const INITIAL_BASE_REWARD_RATE: u64 = BASE_REWARD_RATE_MIN_THRESHOLD;

/// 允许的最低基础奖励率，低于此值时应提高最低难度。
pub const BASE_REWARD_RATE_MIN_THRESHOLD: u64 = 2u64.pow(5);

/// 允许的最高基础奖励率，超过此值时应降低最低难度。
pub const BASE_REWARD_RATE_MAX_THRESHOLD: u64 = 2u64.pow(8);

/// 允许的垃圾邮件/活跃性容忍时间（秒）。
pub const TOLERANCE: i64 = 5;

/// 程序初始化时的最低难度。
pub const INITIAL_MIN_DIFFICULTY: u32 = 1;

/// ORE 代币的小数精度。
/// 每个 ORE 代币有 1000 亿个不可分割的单位（称为“谷物”）。
pub const TOKEN_DECIMALS: u8 = 11;

/// ORE v1 代币的小数精度。
pub const TOKEN_DECIMALS_V1: u8 = 9;

/// 一个 ORE 代币，按不可分割单位计。
pub const ONE_ORE: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// 一分钟的持续时间（秒）。
pub const ONE_MINUTE: i64 = 60;

/// 程序周期中的分钟数。
pub const EPOCH_MINUTES: i64 = 5;

/// 程序周期的持续时间（秒）。
pub const EPOCH_DURATION: i64 = ONE_MINUTE * EPOCH_MINUTES;

/// 最大代币供应量（2100 万）。
pub const MAX_SUPPLY: u64 = ONE_ORE * 21_000_000;

/// 每个周期目标挖掘的 ORE 数量。
pub const TARGET_EPOCH_REWARDS: u64 = ONE_ORE * EPOCH_MINUTES as u64;

/// 每个周期可以挖掘的最大 ORE 数量。
/// 通货膨胀目标 ≈ 每分钟 1 ORE
pub const MAX_EPOCH_REWARDS: u64 = TARGET_EPOCH_REWARDS * BUS_COUNT as u64;

/// 每个 bus 允许在每个周期发行的 ORE 数量。
pub const BUS_EPOCH_REWARDS: u64 = MAX_EPOCH_REWARDS / BUS_COUNT as u64;

/// bus 账户的数量，用于并行化挖矿操作。
pub const BUS_COUNT: usize = 8;

/// 奖励率变化的平滑因子。奖励率在一个周期到下一个周期之间的变化不能超过此常量的倍数。
pub const SMOOTHING_FACTOR: u64 = 2;

// 断言 MAX_EPOCH_REWARDS 能被 BUS_COUNT 整除。
static_assertions::const_assert!(
    (MAX_EPOCH_REWARDS / BUS_COUNT as u64) * BUS_COUNT as u64 == MAX_EPOCH_REWARDS
);

/// bus 账户 PDA 的种子。
pub const BUS: &[u8] = b"bus";

/// config 账户 PDA 的种子。
pub const CONFIG: &[u8] = b"config";

/// 元数据账户 PDA 的种子。
pub const METADATA: &[u8] = b"metadata";

/// mint 账户 PDA 的种子。
pub const MINT: &[u8] = b"mint";

/// proof 账户 PDA 的种子。
pub const PROOF: &[u8] = b"proof";

/// 财库账户 PDA 的种子。
pub const TREASURY: &[u8] = b"treasury";

/// 用于派生 mint PDA 的噪声
pub const MINT_NOISE: [u8; 16] = [
    89, 157, 88, 232, 243, 249, 197, 132, 199, 49, 19, 234, 91, 94, 150, 41,
];

/// 代币元数据的名称。
pub const METADATA_NAME: &str = "ORE";

/// 代币元数据的股票符号。
pub const METADATA_SYMBOL: &str = "ORE";

/// 代币元数据的 URI。
pub const METADATA_URI: &str = "https://ore.supply/metadata-v2.json";

/// 用于常量 PDA 派生的程序 ID
const PROGRAM_ID: [u8; 32] = unsafe { *(&crate::id() as *const Pubkey as *const [u8; 32]) };

/// bus 账户的地址。
pub const BUS_ADDRESSES: [Pubkey; BUS_COUNT] = array_const_fn_init![const_bus_address; 8];

/// 派生常量 bus 地址的函数。
const fn const_bus_address(i: usize) -> Pubkey {
    Pubkey::new_from_array(ed25519::derive_program_address(&[BUS, &[i as u8]], &PROGRAM_ID).0)
}

/// config 账户的地址。
pub const CONFIG_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[CONFIG], &PROGRAM_ID).0);

/// mint 元数据账户的地址。
pub const METADATA_ADDRESS: Pubkey = Pubkey::new_from_array(
    ed25519::derive_program_address(
        &[
            METADATA,
            unsafe { &*(&mpl_token_metadata::ID as *const Pubkey as *const [u8; 32]) },
            unsafe { &*(&MINT_ADDRESS as *const Pubkey as *const [u8; 32]) },
        ],
        unsafe { &*(&mpl_token_metadata::ID as *const Pubkey as *const [u8; 32]) },
    )
        .0,
);

/// mint 账户的地址。
pub const MINT_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[MINT, &MINT_NOISE], &PROGRAM_ID).0);

/// v1 mint 账户的地址。
pub const MINT_V1_ADDRESS: Pubkey = pubkey!("oreoN2tQbHXVaZsr3pf66A48miqcBXCDJozganhEJgz");

/// 财库账户的地址。
pub const TREASURY_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[TREASURY], &PROGRAM_ID).0);

/// 财库账户的 bump 值，用于 CPI 调用。
pub const TREASURY_BUMP: u8 = ed25519::derive_program_address(&[TREASURY], &PROGRAM_ID).1;

/// 财库代币账户的地址。
pub const TREASURY_TOKENS_ADDRESS: Pubkey = Pubkey::new_from_array(
    ed25519::derive_program_address(
        &[
            unsafe { &*(&TREASURY_ADDRESS as *const Pubkey as *const [u8; 32]) },
            unsafe { &*(&spl_token::id() as *const Pubkey as *const [u8; 32]) },
            unsafe { &*(&MINT_ADDRESS as *const Pubkey as *const [u8; 32]) },
        ],
        unsafe { &*(&spl_associated_token_account::id() as *const Pubkey as *const [u8; 32]) },
    )
        .0,
);

/// CU 优化的 Solana 空操作程序的地址。
pub const NOOP_PROGRAM_ID: Pubkey = pubkey!("noop8ytexvkpCuqbf6FB89BSuNemHtPRqaNC31GWivW");