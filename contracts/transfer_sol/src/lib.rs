#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint {
    use {
        solana_program::{
            account_info::{AccountInfo, next_account_info},
            entrypoint,
            entrypoint::ProgramResult,
            msg,
            program::invoke,
            program_error::ProgramError,
            pubkey::Pubkey,
            system_instruction,
        },
        std::convert::TryInto,
    };

    entrypoint!(process_instruction);

    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let payer = next_account_info(accounts_iter)?;
        let payee = next_account_info(accounts_iter)?;

        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;

        msg!(
            "Received request to transfer {:?} lamports from payer ({:?}) to payee ({:?}) ...",
            amount,
            payer.key,
            payee.key
        );
        msg!("Transferring...");

        invoke(
            &system_instruction::transfer(payer.key, payee.key, amount),
            &[payer.clone(), payee.clone()],
        )?;

        msg!("Transfer completed successfully.");
        Ok(())
    }
}
