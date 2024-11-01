use std::mem::size_of;

use luckycoin_api::{
    consts::*,
    instruction::*,
    loaders::*,
    state::{Bus, Config, Treasury},
};
use solana_program::{self, account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError, program_pack::Pack, system_program, sysvar};
use spl_token::state::Mint;
use steel::*;
use luckycoin_api::cpi::create_pda;
use luckycoin_api::loaders::load_signer;

pub fn process_initialize(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // 解析传入的数据为 Initialize 结构体。
    let args = Initialize::try_from_bytes(data)?;
    // 加载账户。
    // 将传入的账户解构为具体的变量。
    let [
        signer,
        bus_0_info,
        bus_1_info,
        bus_2_info,
        bus_3_info,
        bus_4_info,
        bus_5_info,
        bus_6_info,
        bus_7_info,
        config_info,
        metadata_info,
        mint_info,
        treasury_info,
        treasury_tokens_info,
        system_program,
        token_program,
        associated_token_program,
        metadata_program,
        rent_sysvar
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys); 
    };
    // 检查账户是否为签名者
    load_signer(signer)?;

    // 加载未初始化的PDA(程序派生地址)以存储总线程数据
    load_uninitialized_pda(
        bus_0_info, 
        &[BUS, &[0]], 
        args.bus_0_bump, 
        &luckycoin_api::id()
    )?;
    load_uninitialized_pda(
        bus_1_info, 
        &[BUS, &[1]], 
        args.bus_1_bump, 
        &luckycoin_api::id()
    )?;
    load_uninitialized_pda(
        bus_2_info, 
        &[BUS, &[2]],
        args.bus_2_bump, 
        &luckycoin_api::id()
    )?;
    load_uninitialized_pda(
        bus_3_info, 
        &[BUS, &[3]], 
        args.bus_3_bump, 
        &luckycoin_api::id()
    )?;
    load_uninitialized_pda(
        bus_4_info, 
        &[BUS, &[4]], 
        args.bus_4_bump, 
        &luckycoin_api::id()
    )?;
    load_uninitialized_pda(
        bus_5_info, 
        &[BUS, &[5]], 
        args.bus_5_bump, 
        &luckycoin_api::id()
    )?;
    load_uninitialized_pda(
        bus_6_info, 
        &[BUS, &[6]], 
        args.bus_6_bump, 
        &luckycoin_api::id()
    )?;
    load_uninitialized_pda(
        bus_7_info, 
        &[BUS, &[7]], 
        args.bus_7_bump, 
        &luckycoin_api::id()
    )?;
    load_uninitialized_pda(
        config_info, 
        &[CONFIG], 
        args.config_bump, 
        &luckycoin_api::id()
    )?;

    load_uninitialized_pda(
        metadata_info, 
        &[METADATA, mpl_token_metadata::ID.as_ref(), MINT_ADDRESS.as_ref(), ], 
        args.metadata_bump, 
        &mpl_token_metadata::ID
    )?;
    load_uninitialized_pda(
        mint_info, 
        &[MINT, MINT_NOISE.as_slice()], 
        args.mint_bump, 
        &luckycoin_api::id()
    )?;
    load_uninitialized_pda(
        treasury_info, 
        &[TREASURY], 
        args.treasury_bump, 
        &luckycoin_api::id()
    )?;

    load_system_account(
        treasury_tokens_info, 
        true
    )?;
    load_program(
        system_program, 
        system_program::id()
    )?;
    load_program(
        token_program, 
        spl_token::id()
    )?;
    load_program(
        associated_token_program, 
        spl_associated_token_account::id()
    )?;

    load_program(
        metadata_program, 
        mpl_token_metadata::ID
    )?;
    load_sysvar(
        rent_sysvar, 
        sysvar::rent::id()
    )?;
    if signer.key.ne(&INITIALIZER_ADDRESS) {
        return Err(ProgramError::MissingRequiredSignature); 
    }

    let bus_infos = [
        bus_0_info, bus_1_info, bus_2_info, bus_3_info, bus_4_info, bus_5_info, bus_6_info,
        bus_7_info,
    ];
    let bus_bumps = [
        args.bus_0_bump,
        args.bus_1_bump,
        args.bus_2_bump,
        args.bus_3_bump,
        args.bus_4_bump,
        args.bus_5_bump,
        args.bus_6_bump,
        args.bus_7_bump,
    ];

    for i in 0..BUS_COUNT {
        create_pda(bus_infos[i], &luckycoin_api::id(), 8 + size_of::<Bus>(), &[BUS, &[i as u8], &[bus_bumps[i]]], system_program, signer)?;
        let mut bus_data = bus_infos[i].try_borrow_mut_data()?;
        bus_data[0] = Bus::discriminator() as u8; /
        let bus = Bus::try_from_bytes_mut(&mut bus_data)?;
        bus.id = i as u64; 
        bus.rewards = 0; 
        bus.theoretical_rewards = 0; 
        bus.top_balance = 0; 
    }
    create_pda(
        config_info, 
        &luckycoin_api::id(), 
        8 + size_of::<Config>(),
         &[CONFIG, 
         &[args.config_bump]], 
         system_program, 
         signer
    )?;

    // 设置配置账户的初始数据
    let mut config_data = config_info.data.borrow_mut();
    config_data[0] = Config::discriminator() as u8; 
    let config = Config::try_from_bytes_mut(&mut config_data)?;
    config.base_reward_rate = INITIAL_BASE_REWARD_RATE; 
    config.last_reset_at = 0; 
    config.min_difficulty = INITIAL_MIN_DIFFICULTY as u64; 
    config.top_balance = 0; 

    // 初始化国库账户
    msg!("load init treasury account.....");
    create_pda(
        treasury_info, 
        &luckycoin_api::id(), 8 + size_of::<Treasury>(), 
        &[TREASURY, &[args.treasury_bump]], 
        system_program, 
        signer
    )?;

    let mut treasury_data = treasury_info.data.borrow_mut();
    treasury_data[0] = Treasury::discriminator() as u8;
    drop(treasury_data);

    create_pda(
        mint_info, 
        &spl_token::id(), 
        Mint::LEN, 
        &[MINT, MINT_NOISE.as_slice(), 
        &[args.mint_bump]], 
        system_program, 
        signer
    )?;

    solana_program::program::invoke_signed(
        &spl_token::instruction::initialize_mint(
            &spl_token::id(), 
            mint_info.key, 
            treasury_info.key, 
            None,
             TOKEN_DECIMALS
        )?,
        &[token_program.clone(),
            mint_info.clone(),
            treasury_info.clone(),
            rent_sysvar.clone(),
        ],
        &[&[MINT, MINT_NOISE.as_slice(), &[args.mint_bump]]],
    )?;

    // 初始化铸造元数据。
    msg!("load mpl_token_metadata account.....");
    mpl_token_metadata::instructions::CreateMetadataAccountV3Cpi {
        __program: metadata_program,
        metadata: metadata_info,
        mint: mint_info,
        mint_authority: treasury_info,
        payer: signer,
        update_authority: (signer, true),
        system_program,
        rent: Some(rent_sysvar),
        __args: mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs {
            data: mpl_token_metadata::types::DataV2 {
                name: METADATA_NAME.to_string(),
                symbol: METADATA_SYMBOL.to_string(),
                uri: METADATA_URI.to_string(),
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: None,
        },
    }.invoke_signed(&[&[TREASURY, &[args.treasury_bump]]])?;

    create_associated_token_account(
        signer, 
        treasury_info, 
        treasury_tokens_info, 
        mint_info, 
        system_program, 
        token_program, 
        associated_token_program
    )?;

    Ok(())
}
