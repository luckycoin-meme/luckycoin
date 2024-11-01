use drillx::Solution;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

use crate::{
    consts::*,
    instruction::*,
    state::{bus_pda, config_pda, proof_pda, treasury_pda},
};

/// Builds an auth instruction.
pub fn auth(proof: Pubkey) -> Instruction {
    Instruction {
        program_id: NOOP_PROGRAM_ID,
        accounts: vec![],
        data: proof.to_bytes().to_vec(),
    }
}

/// Builds a claim instruction.
pub fn claim(signer: Pubkey, beneficiary: Pubkey, amount: u64) -> Instruction {
    let proof = proof_pda(signer).0;
    let treasury_tokens = spl_associated_token_account::get_associated_token_address(
        &TREASURY_ADDRESS,
        &MINT_ADDRESS,
    );
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(beneficiary, false),
            AccountMeta::new(proof, false),
            AccountMeta::new_readonly(TREASURY_ADDRESS, false),
            AccountMeta::new(treasury_tokens, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: Claim {
            amount: amount.to_le_bytes(),
        }
            .to_bytes(),
    }
}

/// Builds a health instruction.
pub fn health(signer: Pubkey) -> Instruction {
    let proof = proof_pda(signer).0;
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(proof, false),
        ],
        data: Health {}.to_bytes(),
    }
}

/// Builds a close instruction.
pub fn close(signer: Pubkey) -> Instruction {
    let proof = proof_pda(signer).0;
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(proof, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: Close {}.to_bytes(),
    }
}

/// 构建一个挖矿指令
pub fn mine(signer: Pubkey, authority: Pubkey, bus: Pubkey, solution: Solution) -> Instruction {
    // 获取与authority相关的proof PDA(程序派生地址)
    let proof = proof_pda(authority).0;
    // 创建并返回一个新的指令
    Instruction {
        // 指定要调用的智能合约的程序ID
        program_id: crate::id(),
        // 指定此指令所需的账户列表
        accounts: vec![
            // 签名者账户，必须提供签名
            AccountMeta::new(signer, true),
            // bus账户，不需要提供签名
            AccountMeta::new(bus, false),
            // 配置地址账户，只读，不需要提供签名
            AccountMeta::new_readonly(CONFIG_ADDRESS, false),
            // proof PDA 账户，不需要提供签名
            AccountMeta::new(proof, false),
            // 系统变量账户，用于读取指令信息，只读，不需要提供签名
            AccountMeta::new_readonly(sysvar::instructions::id(), false),
            // 系统变量账户，用于读取槽哈希信息，只读，不需要提供签名
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
        ],
        // 将解决方案中的 digest 和 nonce 转换为字节数组
        data: Mine {
            digest: solution.d,
            nonce: solution.n,
        }.to_bytes(), // 将 Mine 结构体转换为字节数组
    }
}

/// 构建一个打开指令。
pub fn open(signer: Pubkey, miner: Pubkey, payer: Pubkey) -> Instruction {
    // 获取与 signer 相关的 proof PDA（程序派生地址）
    let proof_pda = proof_pda(signer);

    // 创建并返回一个新的指令
    Instruction {
        // 指定要调用的智能合约的程序 ID
        program_id: crate::id(),

        // 指定此指令所需的账户列表
        accounts: vec![
            // 签名者账户，必须提供签名
            AccountMeta::new(signer, true),
            // 矿工账户，只读，不需要提供签名
            AccountMeta::new_readonly(miner, false),
            // 付款者账户，必须提供签名
            AccountMeta::new(payer, true),
            // proof PDA 账户，不需要提供签名
            AccountMeta::new(proof_pda.0, false),
            // 系统程序账户，用于与 Solana 系统交互，只读，不需要提供签名
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            // 系统变量账户，用于读取槽哈希信息，只读，不需要提供签名
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
        ],
        // 将 proof PDA 的 bump 值转换为字节数组
        data: Open { bump: proof_pda.1 }.to_bytes(),
    }
}

/// Builds a reset instruction.
pub fn reset(signer: Pubkey) -> Instruction {
    let treasury_tokens = spl_associated_token_account::get_associated_token_address(
        &TREASURY_ADDRESS,
        &MINT_ADDRESS,
    );
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(BUS_ADDRESSES[0], false),
            AccountMeta::new(BUS_ADDRESSES[1], false),
            AccountMeta::new(BUS_ADDRESSES[2], false),
            AccountMeta::new(BUS_ADDRESSES[3], false),
            AccountMeta::new(BUS_ADDRESSES[4], false),
            AccountMeta::new(BUS_ADDRESSES[5], false),
            AccountMeta::new(BUS_ADDRESSES[6], false),
            AccountMeta::new(BUS_ADDRESSES[7], false),
            AccountMeta::new(CONFIG_ADDRESS, false),
            AccountMeta::new(MINT_ADDRESS, false),
            AccountMeta::new(TREASURY_ADDRESS, false),
            AccountMeta::new(treasury_tokens, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: Reset {}.to_bytes(),
    }
}

/// Build a stake instruction.
pub fn stake(signer: Pubkey, sender: Pubkey, amount: u64) -> Instruction {
    let proof = proof_pda(signer).0;
    let treasury_tokens = spl_associated_token_account::get_associated_token_address(
        &TREASURY_ADDRESS,
        &MINT_ADDRESS,
    );
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(proof, false),
            AccountMeta::new(sender, false),
            AccountMeta::new(treasury_tokens, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: Stake {
            amount: amount.to_le_bytes(),
        }
            .to_bytes(),
    }
}

// Build an update instruction.
pub fn update(signer: Pubkey, miner: Pubkey) -> Instruction {
    let proof = proof_pda(signer).0;
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(miner, false),
            AccountMeta::new(proof, false),
        ],
        data: Update {}.to_bytes(),
    }
}

// Build an upgrade instruction.
pub fn upgrade(signer: Pubkey, beneficiary: Pubkey, sender: Pubkey, amount: u64) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(beneficiary, false),
            AccountMeta::new(MINT_ADDRESS, false),
            AccountMeta::new(MINT_V1_ADDRESS, false),
            AccountMeta::new(sender, false),
            AccountMeta::new(TREASURY_ADDRESS, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: Upgrade {
            amount: amount.to_le_bytes(),
        }
            .to_bytes(),
    }
}
/// 构建初始化指令。
pub fn initialize(signer: Pubkey) -> Instruction {
    // 数组，用于存储公共总线 PDA（程序派生地址）
    let bus_pdas = [
        bus_pda(0),
        bus_pda(1),
        bus_pda(2),
        bus_pda(3),
        bus_pda(4),
        bus_pda(5),
        bus_pda(6),
        bus_pda(7),
    ];

    // 获取配置 PDA
    let config_pda = config_pda();

    // 使用程序地址派生找到铸币 PDA
    let mint_pda = Pubkey::find_program_address(&[MINT, MINT_NOISE.as_slice()], &crate::id());

    // 获取财政 PDA
    let treasury_pda = treasury_pda();

    // 获取财政的关联代币地址
    let treasury_tokens = spl_associated_token_account::get_associated_token_address(&treasury_pda.0, &mint_pda.0);

    // 使用程序地址派生找到元数据 PDA
    let metadata_pda = Pubkey::find_program_address(
        &[
            METADATA,
            mpl_token_metadata::ID.as_ref(),
            mint_pda.0.as_ref(),
        ],
        &mpl_token_metadata::ID,
    );

    // 构造指令
    Instruction {
        program_id: crate::id(), // 程序的 ID
        accounts: vec![
            AccountMeta::new(signer, true), // 指令的签名者
            // 将总线 PDA 添加到指令中
            AccountMeta::new(bus_pdas[0].0, false),
            AccountMeta::new(bus_pdas[1].0, false),
            AccountMeta::new(bus_pdas[2].0, false),
            AccountMeta::new(bus_pdas[3].0, false),
            AccountMeta::new(bus_pdas[4].0, false),
            AccountMeta::new(bus_pdas[5].0, false),
            AccountMeta::new(bus_pdas[6].0, false),
            AccountMeta::new(bus_pdas[7].0, false),
            AccountMeta::new(config_pda.0, false), // 配置 PDA
            AccountMeta::new(metadata_pda.0, false), // 元数据 PDA
            AccountMeta::new(mint_pda.0, false), // 铸币 PDA
            AccountMeta::new(treasury_pda.0, false), // 财政 PDA
            AccountMeta::new(treasury_tokens, false), // 财政代币关联地址
            // 只读账户（此指令不会修改）
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: Initialize {
            // 每个总线 PDA 的 bump 值
            bus_0_bump: bus_pdas[0].1,
            bus_1_bump: bus_pdas[1].1,
            bus_2_bump: bus_pdas[2].1,
            bus_3_bump: bus_pdas[3].1,
            bus_4_bump: bus_pdas[4].1,
            bus_5_bump: bus_pdas[5].1,
            bus_6_bump: bus_pdas[6].1,
            bus_7_bump: bus_pdas[7].1,
            // 配置 PDA 的 bump 值
            config_bump: config_pda.1,
            // 元数据 PDA 的 bump 值
            metadata_bump: metadata_pda.1,
            // 铸币 PDA 的 bump 值
            mint_bump: mint_pda.1,
            // 财政 PDA 的 bump 值
            treasury_bump: treasury_pda.1,
        }.to_bytes(), // 将 Initialize 数据转换为字节以用于指令
    }
}
