use {
    borsh::BorshDeserialize,
    solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
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
    msg!("minting...1");
    let data = TokenData::try_from_slice(instructions_data)?;
    msg!("minting...2");
    let minter = Minter::try_new(accounts, data)?;
    msg!("minting...3");
    minter.run()?;
    msg!("Token mint process completed successfully.");
    Ok(())
}
