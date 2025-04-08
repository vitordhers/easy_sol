use anchor_lang::{prelude::*, solana_program::program::invoke, system_program::{create_account, CreateAccount}};

use crate::{
    data::{FungibleAssetMetadata, FungibleTokenMetadata, NonFungibleTokenMetadata},
    FungibleAccounts, MintFungible, NonFungibleAccounts, Programs,
};

pub enum Minter<'a> {
    FungibleToken {
        accounts: FungibleAccounts<'a>,
        metadata: &'a FungibleTokenMetadata,
        programs: Programs<'a>,
    },
    FungibleAsset {
        accounts: FungibleAccounts<'a>,
        metadata: &'a FungibleAssetMetadata,
        programs: Programs<'a>,
    },
    NonFungibleToken {
        accounts: NonFungibleAccounts<'a>,
        metadata: &'a NonFungibleTokenMetadata,
        programs: Programs<'a>,
    },
}

impl<'a> Minter<'a> {
    fn new_fungible_token(
        accounts: &'a MintFungible<'a>,
        metadata: &'a FungibleTokenMetadata,
    ) -> Self {
        Self::FungibleToken {
            accounts: accounts.into(),
            metadata,
            programs: accounts.into(),
        }
    }

    fn create_mint_account(&self) -> Result<(), ProgramError> {
        msg!("Creating mint account...");
        let rent = Rent::get()?;
        let account_data_size = 82;
        let rent_exemption = rent.minimum_balance(account_data_size);
        let away = match self {
            Self::FungibleToken { accounts, .. } => accounts,
            Self::FungibleAsset { accounts, .. } => accounts,
            Self::NonFungibleToken { accounts, .. } => accounts
        };
        let accounts = CreateAccount {from: self. }
        let ctx = CpiContext::
        let instruction = create_account(
            self.accounts.mint_authority.key,
            self.accounts.mint.key,
            rent_exemption,
            account_data_size as u64,
            self.programs.token.key,
        );
        let account_infos = [
            self.accounts.mint.clone(),
            self.accounts.mint_authority.clone(),
            self.programs.token.clone(),
        ];

        invoke(&instruction, &account_infos)?;
        msg!("Account created successfully! {}", self.accounts.mint.key);
        Ok(())
    }
}
