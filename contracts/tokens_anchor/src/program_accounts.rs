use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::Token,
};

#[derive(Accounts)]
pub struct MintFungible<'info> {
    #[account(mut)]
    pub mint: Signer<'info>,
    #[account(mut)]
    /// Check: this will be created on-chain
    pub token: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    /// Check: this will be created on-chain
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
}

#[derive(Accounts)]
pub struct MintNonFungible<'info> {
    #[account(mut)]
    pub mint: Signer<'info>,
    #[account(mut)]
    /// Check: this will be created on-chain
    pub token: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    /// Check: this will be created on-chain
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// Check: this will be created on-chain
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
}

pub struct MinterAccounts<'a, 'b> {
    pub mint: &'b Signer<'a>,
    pub token: &'b UncheckedAccount<'a>,
    pub mint_authority: &'b Signer<'a>,
    pub metadata: &'b UncheckedAccount<'a>,
    pub master_edition: Option<&'b UncheckedAccount<'a>>,
}

impl<'a, 'b> From<&'b MintNonFungible<'a>> for MinterAccounts<'a, 'b> {
    fn from(value: &'b MintNonFungible<'a>) -> Self {
        Self {
            mint: &value.mint,
            token: &value.token,
            mint_authority: &value.mint_authority,
            metadata: &value.metadata,
            master_edition: Some(&value.master_edition),
        }
    }
}

impl<'a, 'b> From<&'b MintFungible<'a>> for MinterAccounts<'a, 'b> {
    fn from(value: &'b MintFungible<'a>) -> Self {
        Self {
            mint: &value.mint,
            token: &value.token,
            mint_authority: &value.mint_authority,
            metadata: &value.metadata,
            master_edition: None,
        }
    }
}

pub struct MinterPrograms<'a, 'b> {
    pub rent: &'b Sysvar<'a, Rent>,
    pub system: &'b Program<'a, System>,
    pub token: &'b Program<'a, Token>,
    pub associated_token: &'b Program<'a, AssociatedToken>,
    pub metadata: &'b Program<'a, Metadata>,
}

impl<'a, 'b> From<&'b MintFungible<'a>> for MinterPrograms<'a, 'b> {
    fn from(value: &'b MintFungible<'a>) -> Self {
        Self {
            rent: &value.rent,
            system: &value.system_program,
            token: &value.token_program,
            associated_token: &value.associated_token_program,
            metadata: &value.metadata_program,
        }
    }
}

impl<'a, 'b> From<&'b MintNonFungible<'a>> for MinterPrograms<'a, 'b> {
    fn from(value: &'b MintNonFungible<'a>) -> Self {
        Self {
            rent: &value.rent,
            system: &value.system_program,
            token: &value.token_program,
            associated_token: &value.associated_token_program,
            metadata: &value.metadata_program,
        }
    }
}
