use anchor_lang::{prelude::Pubkey, AnchorDeserialize, AnchorSerialize};
use anchor_spl::metadata::mpl_token_metadata::types::{
    Collection, Creator, DataV2, UseMethod, Uses,
};
use std::str::FromStr;

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct FungibleTokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct FungibleAssetMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub uses: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct NonFungibleTokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators_addresses: Option<Vec<String>>,
    pub collection_address: Option<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct FungibleTokenParams {
    pub decimals: u8,
    pub initial_supply: u64,
    pub should_freeze_after_mint: bool,
    pub metadata: FungibleTokenMetadata,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct FungibleAssetParams {
    pub decimals: u8,
    pub quantity: u64,
    pub metadata: FungibleAssetMetadata,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct NonFungibleTokenParams {
    pub metadata: NonFungibleTokenMetadata,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub enum TokenData {
    Fungible(FungibleTokenParams),
    FungibleAsset(FungibleAssetParams),
    NonFungible(NonFungibleTokenParams),
}

impl From<&TokenData> for DataV2 {
    fn from(value: &TokenData) -> Self {
        match value {
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
                            share: 100 / addresses.len() as u8,
                            address: Pubkey::from_str(addr)
                                .expect("address to be convertable to pubkey"),
                            verified: false,
                        })
                        .collect()
                }),
                collection: metadata
                    .collection_address
                    .clone()
                    .map(|collection_addr| Collection {
                        verified: false,
                        key: Pubkey::from_str(collection_addr.as_str())
                            .expect("collection address to be convertable to pubkey"),
                    }),
                seller_fee_basis_points: metadata.seller_fee_basis_points,
            },
        }
    }
}
