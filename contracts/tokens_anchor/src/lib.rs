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

    pub fn mint_nft(ctx: Context<MintNonFungible>, data: TokenData) -> Result<()> {
        let accounts: &MintNonFungible = &ctx.accounts;
        let minter = Minter::from(accounts);
        minter.run(&data)?;
        Ok(())
    }
}
