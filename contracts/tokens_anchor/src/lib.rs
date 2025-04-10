use anchor_lang::prelude::*;

declare_id!("5HYopUu3aUZ3CQC9AMq3iKa5Q9awF6aEm18jvHfKQ45r");

mod program_accounts;
pub use program_accounts::*;
mod data;
use data::*;
mod minter;
use minter::*;

#[program]
pub mod tokens_anchor {
    use super::*;

    pub fn mint_fungible_token(ctx: Context<MintFungible>, data: TokenData) -> Result<()> {
        if !matches!(data, TokenData::Fungible(_)) {
            return Err(Error::ProgramError(Box::new(ProgramErrorWithOrigin {
                program_error: ProgramError::InvalidInstructionData,
                error_origin: None,
                compared_values: None,
            })));
        }
        let minter = Minter::from(&*ctx.accounts);
        minter.run(&data)?;
        Ok(())
    }

    pub fn mint_fungible_asset(ctx: Context<MintFungible>, data: TokenData) -> Result<()> {
        if !matches!(data, TokenData::FungibleAsset(_)) {
            return Err(Error::ProgramError(Box::new(ProgramErrorWithOrigin {
                program_error: ProgramError::InvalidInstructionData,
                error_origin: None,
                compared_values: None,
            })));
        }
        let minter = Minter::from(&*ctx.accounts);
        minter.run(&data)?;
        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNonFungible>, data: TokenData) -> Result<()> {
        if !matches!(data, TokenData::NonFungible(_)) {
            return Err(Error::ProgramError(Box::new(ProgramErrorWithOrigin {
                program_error: ProgramError::InvalidInstructionData,
                error_origin: None,
                compared_values: None,
            })));
        }
        let minter = Minter::from(&*ctx.accounts);
        minter.run(&data)?;
        Ok(())
    }
}
