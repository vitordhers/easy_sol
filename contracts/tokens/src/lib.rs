use {
    borsh::BorshDeserialize,
    solana_program::{
        account_info::{AccountInfo, next_account_info},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

mod minter;
pub mod data;

use minter::*;

entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instructions_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != program_id {
        msg!("Account doesn't have correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    let data = TokenData::try_from_slice(instructions_data)?;
    let minter = Minter::try_new(accounts, data)?;
    minter.run()?;
    msg!("Token mint process completed successfully.");
    Ok(())
}
