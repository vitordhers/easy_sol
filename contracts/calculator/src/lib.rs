#![allow(unexpected_cfgs)]
mod calculator;
use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use calculator::*;
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Calculator {
    pub value: f32,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != program_id {
        msg!("Account doesn't have correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    let mut calc = Calculator::try_from_slice(&account.data.borrow())?;
    let calculator_ix = CalculatorInstructions::try_from_slice(instructions_data)?;
    msg!("Received calculator ix ${:?}", calculator_ix);
    calc.value = calculator_ix.evaluate(calc.value);
    calc.serialize(&mut &mut account.data.borrow_mut()[..])?;
    msg!("Result is: {}", calc.value);
    Ok(())
}
