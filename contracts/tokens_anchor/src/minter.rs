use crate::{data::TokenData, MintFungible, MintNonFungible, MinterAccounts, MinterPrograms};
use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::{
    associated_token::{create as create_token_account, Create as CreateTokenAccount},
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3,
    },
    token::{freeze_account, initialize_mint, mint_to, FreezeAccount, InitializeMint, MintTo},
};
use mpl_token_metadata::types::DataV2;

pub struct Minter<'a> {
    accounts: &'a MinterAccounts<'a>,
    programs: &'a MinterPrograms<'a>,
}

impl<'a> From<MintFungible<'a>> for Minter<'a> {
    fn from(value: MintFungible<'a>) -> Self {
        Self {
            accounts: &value.into(),
            programs: &value.into(),
        }
    }
}

impl<'a> From<&'a MintNonFungible<'a>> for Minter<'a> {
    fn from(value: &'a MintNonFungible<'a>) -> Self {
        Self {
            accounts: value.into(),
            programs: value.into(),
        }
    }
}

impl Minter<'_> {
    fn create_mint_account(&self) -> Result<()> {
        msg!("Creating mint account...");
        let rent = Rent::get()?;
        let account_data_size = 82;
        let rent_exemption_lamports = rent.minimum_balance(account_data_size);
        let accounts = CreateAccount {
            from: self.accounts.mint_authority.to_account_info(),
            to: self.accounts.mint.to_account_info(),
        };
        let ctx = CpiContext::new(self.programs.token.to_account_info(), accounts);
        create_account(
            ctx,
            rent_exemption_lamports,
            account_data_size as u64,
            &self.accounts.mint_authority.key(),
        )?;
        msg!(
            "Mint account created successfully! {}",
            self.accounts.mint.key
        );
        Ok(())
    }

    fn initialize_mint(&self, data: &TokenData) -> Result<()> {
        msg!("Initializing mint account...");
        let accounts = InitializeMint {
            mint: self.accounts.mint.to_account_info(),
            rent: self.programs.rent.to_account_info(),
        };
        let ctx = CpiContext::new(self.programs.token.to_account_info(), accounts);
        let decimals = match data {
            TokenData::Fungible(data) => data.decimals,
            TokenData::FungibleAsset(data) => data.decimals,
            TokenData::NonFungible(_) => 0,
        };
        initialize_mint(
            ctx,
            decimals,
            self.accounts.mint_authority.key,
            Some(self.accounts.mint_authority.key),
        )?;
        Ok(())
    }

    fn create_token_account(&self) -> Result<()> {
        msg!("Creating token account...");
        let accounts = CreateTokenAccount {
            payer: self.accounts.mint_authority.to_account_info(),
            associated_token: self.accounts.token.to_account_info(),
            authority: self.accounts.mint_authority.to_account_info(),
            mint: self.accounts.mint.to_account_info(),
            system_program: self.programs.system.to_account_info(),
            token_program: self.programs.token.to_account_info(),
        };
        let ctx = CpiContext::new(self.programs.associated_token.to_account_info(), accounts);
        create_token_account(ctx)?;
        Ok(())
    }

    fn mint(&self, data: &TokenData) -> Result<()> {
        msg!("Minting token...");
        let accounts = MintTo {
            mint: self.accounts.mint.to_account_info(),
            authority: self.accounts.mint_authority.to_account_info(),
            to: self.accounts.token.to_account_info(),
        };
        let ctx = CpiContext::new(self.programs.token.to_account_info(), accounts);
        let amount = match data {
            TokenData::Fungible(data) => data.initial_supply * 10_u64.pow(data.decimals as u32),
            TokenData::FungibleAsset(data) => data.quantity,
            TokenData::NonFungible(_) => 1,
        };
        mint_to(ctx, amount)?;
        Ok(())
    }

    fn freeze(&self) -> Result<()> {
        msg!("Freezing token account...");
        let accounts = FreezeAccount {
            authority: self.accounts.mint_authority.to_account_info(),
            mint: self.accounts.mint.to_account_info(),
            account: self.accounts.token.to_account_info(),
        };
        let ctx = CpiContext::new(self.programs.token.to_account_info(), accounts);
        freeze_account(ctx)?;
        Ok(())
    }

    fn create_metadata_account(&self, data: &TokenData) -> Result<()> {
        let accounts = CreateMetadataAccountsV3 {
            mint: self.accounts.mint.to_account_info(),
            mint_authority: self.accounts.mint_authority.to_account_info(),
            payer: self.accounts.mint_authority.to_account_info(),
            update_authority: self.accounts.mint_authority.to_account_info(),
            metadata: self.accounts.metadata.to_account_info(),
            rent: self.programs.rent.to_account_info(),
            system_program: self.programs.system.to_account_info(),
        };
        let ctx = CpiContext::new(self.programs.metadata.to_account_info(), accounts);
        let data: DataV2 = data.into();
        create_metadata_accounts_v3(ctx, data, false, true, None)?;
        Ok(())
    }

    fn create_master_edition(&self) -> Result<()> {
        let accounts = CreateMasterEditionV3 {
            mint: self.accounts.mint.to_account_info(),
            payer: self.accounts.mint_authority.to_account_info(),
            mint_authority: self.accounts.mint_authority.to_account_info(),
            update_authority: self.accounts.mint_authority.to_account_info(),
            metadata: self.accounts.metadata.to_account_info(),
            edition: self
                .accounts
                .master_edition
                .expect("master edition account to be provided when creating master edition")
                .to_account_info(),
            token_program: self.programs.token.to_account_info(),
            rent: self.programs.rent.to_account_info(),
            system_program: self.programs.system.to_account_info(),
        };
        let ctx = CpiContext::new(self.programs.metadata.to_account_info(), accounts);
        create_master_edition_v3(ctx, None)?;
        Ok(())
    }

    pub fn run(&self, data: &TokenData) -> Result<()> {
        self.create_mint_account()?;
        self.initialize_mint(data)?;
        self.create_token_account()?;
        self.mint(data)?;
        match data {
            TokenData::Fungible(data) => {
                if data.should_freeze_after_mint {
                    self.freeze()?;
                }
            }
            _ => {
                self.freeze()?;
            }
        }
        self.create_metadata_account(data)?;
        self.create_master_edition()?;
        Ok(())
    }
}
