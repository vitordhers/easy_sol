use {
    borsh::BorshDeserialize,
    solana_program::{
        account_info::{AccountInfo, next_account_info},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
    },
};

pub mod data;
mod minter;

use minter::*;

entrypoint!(process_instruction);

fn process_instruction<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instructions_data: &[u8],
) -> ProgramResult {
    let data = TokenData::try_from_slice(instructions_data)?;
    let minter = Minter::try_new(accounts, data)?;
    minter.run()?;
    msg!("Token mint process completed successfully.");
    Ok(())
}
