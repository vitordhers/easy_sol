use anchor_lang::prelude::*;

declare_id!("GCrqR8NeXX7j4ho8nPoJxojSS7deokj7W5mNBEdGSaqC");

mod program_accounts;
pub use program_accounts::*;
pub mod data;
use data::*;
mod minter;
use minter::*;

#[program]
pub mod tokens {
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
