use {
    crate::data::*,
    borsh::{BorshDeserialize, BorshSerialize},
    mpl_token_metadata::{
        ID as MPL_METADATA_ID,
        instructions::{
            CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3,
            CreateMetadataAccountV3InstructionArgs,
        },
        types::{Collection, CollectionDetails, Creator, DataV2, UseMethod, Uses},
    },
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
    metadata: &'a AccountInfo<'a>,
    master_edition: Option<&'a AccountInfo<'a>>,
}

pub struct MinterPrograms<'a> {
    rent: &'a AccountInfo<'a>,
    system: &'a AccountInfo<'a>,
    token: &'a AccountInfo<'a>,
    associated_token: &'a AccountInfo<'a>,
}

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
        let metadata = next_account_info(iter)?;
        let master_edition = match next_account_info(iter) {
            Ok(account) => Some(account),
            Err(_) => None,
        };
        Ok(Self {
            mint,
            token,
            mint_authority,
            metadata,
            master_edition,
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
            Some(self.accounts.mint_authority.key),
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

    fn create_metadata_account(&self) -> Result<(), ProgramError> {
        let data = match &self.data {
            TokenData::Fungible(FungibleTokenParams { metadata, .. }) => DataV2 {
                name: metadata.name.clone(),
                uri: metadata.uri.clone(),
                symbol: metadata.symbol.clone(),
                uses: None,
                creators: None,
                collection: None,
                seller_fee_basis_points: 0,
            },
            TokenData::FungibleAsset(FungibleAssetParams { metadata, .. }) => DataV2 {
                name: metadata.name.clone(),
                uri: metadata.uri.clone(),
                symbol: metadata.symbol.clone(),
                uses: if metadata.uses > 0 {
                    Some(Uses {
                        total: metadata.uses,
                        remaining: metadata.uses,
                        use_method: UseMethod::Burn,
                    })
                } else {
                    None
                },
                creators: None,
                collection: None,
                seller_fee_basis_points: 0,
            },
            TokenData::NonFungible(NonFungibleTokenParams { metadata, .. }) => DataV2 {
                name: metadata.name.clone(),
                uri: metadata.uri.clone(),
                symbol: metadata.symbol.clone(),
                uses: None,
                creators: metadata.creators_addresses.clone().map(|addresses| {
                    addresses
                        .iter()
                        .map(|addr| Creator {
                            share: 0,
                            address: *addr,
                            verified: false,
                        })
                        .collect()
                }),
                collection: metadata
                    .collection_address
                    .map(|collection_addr| Collection {
                        verified: false,
                        key: collection_addr,
                    }),
                seller_fee_basis_points: metadata.seller_fee_basis_points,
            },
        };
        let args = CreateMetadataAccountV3InstructionArgs {
            data,
            is_mutable: false,
            collection_details: None,
        };
        let factory = CreateMetadataAccountV3 {
            metadata: *self.accounts.metadata.key,
            mint: *self.accounts.mint.key,
            payer: *self.accounts.mint_authority.key,
            update_authority: (*self.accounts.mint_authority.key, true),
            mint_authority: *self.accounts.mint_authority.key,
            system_program: *self.programs.system.key,
            rent: Some(*self.programs.rent.key),
        };

        let instruction = factory.instruction(args);
        let account_infos = [
            self.accounts.metadata.clone(),
            self.accounts.mint.clone(),
            self.accounts.token.clone(),
            self.accounts.mint_authority.clone(),
        ];
        invoke(&instruction, &account_infos)?;

        Ok(())
    }

    fn create_master_edition(&self) -> Result<(), ProgramError> {
        match self.data {
            TokenData::NonFungible(_) => {}
            _ => return Ok(()),
        }
        let args = CreateMasterEditionV3InstructionArgs { max_supply: None };
        let master_edition_account = self.accounts.master_edition.unwrap();
        let factory = CreateMasterEditionV3 {
            metadata: *self.accounts.metadata.key,
            mint: *self.accounts.mint.key,
            mint_authority: *self.accounts.mint_authority.key,
            payer: *self.accounts.mint_authority.key,
            update_authority: *self.accounts.mint_authority.key,
            edition: *master_edition_account.key,
            token_program: MPL_METADATA_ID,
            system_program: *self.programs.system.key,
            rent: Some(*self.programs.rent.key),
        };

        let instruction = factory.instruction(args);
        let account_infos = [
            self.accounts.metadata.clone(),
            self.accounts.mint.clone(),
            self.accounts.mint_authority.clone(),
            master_edition_account.clone(),

        ];

        invoke(&instruction, &account_infos)?;

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
        self.mint()?;
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
