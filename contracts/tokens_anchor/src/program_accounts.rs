use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::Token,
};

#[derive(Accounts, Clone)]
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

#[derive(Accounts, Clone)]
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

pub struct MinterAccounts<'info> {
    pub mint: Signer<'info>,
    pub token: UncheckedAccount<'info>,
    pub mint_authority: Signer<'info>,
    pub metadata: UncheckedAccount<'info>,
    pub master_edition: Option<UncheckedAccount<'info>>,
}

impl<'info> From<MintNonFungible<'info>> for MinterAccounts<'info> {
    fn from(value: MintNonFungible<'info>) -> Self {
        Self {
            mint: value.mint,
            token: value.token,
            mint_authority: value.mint_authority,
            metadata: value.metadata,
            master_edition: Some(value.master_edition),
        }
    }
}

impl<'info> From<MintFungible<'info>> for MinterAccounts<'info> {
    fn from(value: MintFungible<'info>) -> Self {
        Self {
            mint: value.mint,
            token: value.token,
            mint_authority: value.mint_authority,
            metadata: value.metadata,
            master_edition: None,
        }
    }
}

pub struct MinterPrograms<'info> {
    pub rent: Sysvar<'info, Rent>,
    pub system: Program<'info, System>,
    pub token: Program<'info, Token>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub metadata: Program<'info, Metadata>,
}

impl<'info> From<MintFungible<'info>> for MinterPrograms<'info> {
    fn from(value: MintFungible<'info>) -> Self {
        Self {
            rent: value.rent,
            system: value.system_program,
            token: value.token_program,
            associated_token: value.associated_token_program,
            metadata: value.metadata_program,
        }
    }
}

impl<'info> From<MintNonFungible<'info>> for MinterPrograms<'info> {
    fn from(value: MintNonFungible<'info>) -> Self {
        Self {
            rent: value.rent,
            system: value.system_program,
            token: value.token_program,
            associated_token: value.associated_token_program,
            metadata: value.metadata_program,
        }
    }
}
