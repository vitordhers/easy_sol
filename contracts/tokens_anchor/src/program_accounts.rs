use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::Token,
};

#[derive(Accounts)]
pub struct MintFungible<'a> {
    #[account(mut)]
    pub mint: Signer<'a>,
    #[account(mut)]
    /// Check: this will be created on-chain
    pub token: UncheckedAccount<'a>,
    #[account(mut)]
    pub mint_authority: Signer<'a>,
    /// Check: this will be created on-chain
    #[account(mut)]
    pub metadata: UncheckedAccount<'a>,
    pub rent: Sysvar<'a, Rent>,
    pub system_program: Program<'a, System>,
    pub token_program: Program<'a, Token>,
    pub associated_token_program: Program<'a, AssociatedToken>,
    pub metadata_program: Program<'a, Metadata>,
}

#[derive(Accounts)]
pub struct MintNonFungible<'a> {
    #[account(mut)]
    mint: Signer<'a>,
    #[account(mut)]
    /// Check: this will be created on-chain
    token: UncheckedAccount<'a>,
    #[account(mut)]
    pub mint_authority: Signer<'a>,
    /// Check: this will be created on-chain
    #[account(mut)]
    pub metadata: UncheckedAccount<'a>,
    /// Check: this will be created on-chain
    #[account(mut)]
    pub master_edition: UncheckedAccount<'a>,
    pub rent: Sysvar<'a, Rent>,
    pub system_program: Program<'a, System>,
    pub token_program: Program<'a, Token>,
    pub associated_token_program: Program<'a, AssociatedToken>,
    pub metadata_program: Program<'a, Metadata>,
}

pub struct Programs<'a> {
    pub rent: &'a Sysvar<'a, Rent>,
    pub system: &'a Program<'a, System>,
    pub token: &'a Program<'a, Token>,
    pub associated_token: &'a Program<'a, AssociatedToken>,
    pub metadata: &'a Program<'a, Metadata>,
}

impl<'a> From<&'a MintNonFungible<'a>> for Programs<'a> {
    fn from(value: &'a MintNonFungible<'a>) -> Self {
        Self {
            rent: &value.rent,
            system: &value.system_program,
            token: &value.token_program,
            associated_token: &value.associated_token_program,
            metadata: &value.metadata_program,
        }
    }
}

impl<'a> From<&'a MintFungible<'a>> for Programs<'a> {
    fn from(value: &'a MintFungible<'a>) -> Self {
        Self {
            rent: &value.rent,
            system: &value.system_program,
            token: &value.token_program,
            associated_token: &value.associated_token_program,
            metadata: &value.metadata_program,
        }
    }
}

pub struct FungibleAccounts<'a> {
    pub mint: &'a Signer<'a>,
    pub token: &'a UncheckedAccount<'a>,
    pub mint_authority: &'a Signer<'a>,
    pub metadata: &'a UncheckedAccount<'a>,
}

impl<'a> From<&'a MintFungible<'a>> for FungibleAccounts<'a> {
    fn from(value: &'a MintFungible<'a>) -> Self {
        Self {
            mint: &value.mint,
            token: &value.token,
            mint_authority: &value.mint_authority,
            metadata: &value.metadata,
        }
    }
}

pub struct NonFungibleAccounts<'a> {
    pub mint: &'a Signer<'a>,
    pub token: &'a UncheckedAccount<'a>,
    pub mint_authority: &'a Signer<'a>,
    pub metadata: &'a UncheckedAccount<'a>,
    pub master_edition: &'a UncheckedAccount<'a>,
}

impl<'a> From<&'a MintNonFungible<'a>> for NonFungibleAccounts<'a> {
    fn from(value: &'a MintNonFungible<'a>) -> Self {
        Self {
            mint: &value.mint,
            token: &value.token,
            mint_authority: &value.mint_authority,
            metadata: &value.metadata,
            master_edition: &value.master_edition,
        }
    }
}
