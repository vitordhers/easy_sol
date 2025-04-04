use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FungibleTokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FungibleAssetMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub uses: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NonFungibleTokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators_addresses: Option<Vec<Pubkey>>,
    pub collection_address: Option<Pubkey>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FungibleTokenParams {
    pub decimals: u8,
    pub initial_supply: u64,
    pub should_freeze_after_mint: bool,
    pub metadata: FungibleTokenMetadata,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FungibleAssetParams {
    pub decimals: u8,
    pub quantity: u64,
    pub metadata: FungibleAssetMetadata,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NonFungibleTokenParams {
    pub metadata: NonFungibleTokenMetadata,
}
