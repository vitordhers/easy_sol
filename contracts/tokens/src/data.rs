use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FungibleTokenParams {
    pub decimals: u8,
    pub initial_supply: u64,
    pub should_freeze_after_mint: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FungibleAssetParams {
    pub decimals: u8,
    pub quantity: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NonFungibleTokenParams {}
