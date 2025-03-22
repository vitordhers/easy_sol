use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{AccountInfo, next_account_info},
        msg,
        program::invoke,
        program_error::ProgramError,
        rent::Rent,
        system_instruction::create_account,
        sysvar::Sysvar,
    },
    spl_associated_token_account::instruction::create_associated_token_account,
    spl_token::instruction::{freeze_account, initialize_mint, mint_to},
    std::slice::Iter,
};

pub struct MinterAccounts<'a> {
    mint: &'a AccountInfo<'a>,
    token: &'a AccountInfo<'a>,
    mint_authority: &'a AccountInfo<'a>,
}

pub struct MinterPrograms<'a> {
    rent: &'a AccountInfo<'a>,
    system: &'a AccountInfo<'a>,
    token: &'a AccountInfo<'a>,
    associated_token: &'a AccountInfo<'a>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FungibleTokenParams {
    decimals: u8,
    initial_supply: u64,
    should_freeze_after_mint: bool,
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FungibleAssetParams {
    decimals: u8,
    quantity: u64,
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NonFungibleTokenParams {}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TokenData {
    Fungible(FungibleTokenParams),
    FungibleAsset(FungibleAssetParams),
    NonFungible(NonFungibleTokenParams),
}

impl<'a> MinterAccounts<'a> {
    fn try_new(iter: &mut Iter<'a, AccountInfo<'a>>) -> Result<Self, ProgramError> {
        let mint = next_account_info(iter)?;
        let token = next_account_info(iter)?;
        let mint_authority = next_account_info(iter)?;
        Ok(Self {
            mint,
            token,
            mint_authority,
        })
    }
}

impl<'a> MinterPrograms<'a> {
    fn try_new(iter: &mut Iter<'a, AccountInfo<'a>>) -> Result<Self, ProgramError> {
        let rent = next_account_info(iter)?;
        let system = next_account_info(iter)?;
        let token = next_account_info(iter)?;
        let associated_token = next_account_info(iter)?;

        Ok(Self {
            rent,
            system,
            token,
            associated_token,
        })
    }
}

pub struct Minter<'a> {
    accounts: MinterAccounts<'a>,
    programs: MinterPrograms<'a>,
    data: TokenData,
}

impl<'a> Minter<'a> {
    fn create_mint_account(&self) -> Result<(), ProgramError> {
        msg!("Creating mint account...");
        let rent = Rent::get()?;
        let account_data_size = 82;
        let rent_exemption = rent.minimum_balance(account_data_size);
        let instruction = create_account(
            &self.accounts.mint_authority.key,
            &self.accounts.mint.key,
            rent_exemption,
            account_data_size as u64,
            &self.programs.token.key,
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

    fn initialize_mint(&self) -> Result<(), ProgramError> {
        msg!("Initializing mint account...");
        let decimals = match &self.data {
            TokenData::Fungible(data) => data.decimals,
            TokenData::FungibleAsset(data) => data.decimals,
            TokenData::NonFungible(_) => 0,
        };
        let instruction = initialize_mint(
            self.programs.token.key,
            self.accounts.mint.key,
            self.accounts.mint_authority.key,
            Some(&self.accounts.mint_authority.key),
            decimals,
        )?;
        let account_infos = [
            self.accounts.mint.clone(),
            self.accounts.mint_authority.clone(),
            self.programs.token.clone(),
            self.programs.rent.clone(),
        ];
        invoke(&instruction, &account_infos)?;
        msg!("Mint account initialized successfully!");
        Ok(())
    }

    fn create_token_account(&self) -> Result<(), ProgramError> {
        msg!("Creating token account...");
        let instruction = create_associated_token_account(
            self.accounts.mint_authority.key,
            self.accounts.mint_authority.key,
            self.accounts.mint.key,
            self.programs.token.key,
        );
        let account_infos = [
            self.accounts.mint.clone(),
            self.accounts.token.clone(),
            self.accounts.mint_authority.clone(),
            self.programs.token.clone(),
            self.programs.associated_token.clone(),
        ];
        invoke(&instruction, &account_infos)?;
        msg!(
            "Token account created successfully! {}",
            self.accounts.token.key
        );
        Ok(())
    }

    fn mint(&self) -> Result<(), ProgramError> {
        msg!("Minting token to token account...");
        let amount = match &self.data {
            TokenData::Fungible(data) => data.initial_supply,
            TokenData::FungibleAsset(data) => data.quantity,
            TokenData::NonFungible(_) => 1,
        };

        let instruction = mint_to(
            self.programs.token.key,
            self.accounts.mint.key,
            self.accounts.token.key,
            self.accounts.mint_authority.key,
            &[self.accounts.mint_authority.key],
            amount,
        )?;
        let account_infos = [
            self.accounts.mint.clone(),
            self.accounts.mint_authority.clone(),
            self.accounts.token.clone(),
            self.programs.token.clone(),
            self.programs.rent.clone(),
        ];
        invoke(&instruction, &account_infos)?;
        msg!("Token minted successfully!");
        Ok(())
    }

    fn freeze(&self) -> Result<(), ProgramError> {
        msg!("Freezing token account...");
        let instruction = freeze_account(
            self.programs.token.key,
            self.accounts.token.key,
            self.accounts.mint.key,
            self.accounts.mint_authority.key,
            &[self.accounts.mint_authority.key],
        )?;
        let account_infos = [
            self.accounts.mint.clone(),
            self.accounts.token.clone(),
            self.accounts.mint_authority.clone(),
        ];
        invoke(&instruction, &account_infos)?;
        msg!("Token account frozen successfuly!");
        Ok(())
    }

    pub fn run(&self) -> Result<(), ProgramError> {
        self.create_mint_account()?;
        self.initialize_mint()?;
        self.create_token_account()?;
        match &self.data {
            TokenData::Fungible(data) => {
                if data.should_freeze_after_mint {
                    self.freeze()?;
                }
            }
            _ => {
                self.freeze()?;
            }
        }
        Ok(())
    }

    pub fn try_new(accounts: &'a [AccountInfo<'a>], data: TokenData) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = MinterAccounts::try_new(accounts_iter)?;
        let programs = MinterPrograms::try_new(accounts_iter)?;
        Ok(Self {
            accounts,
            programs,
            data,
        })
    }
}
