use anchor_lang::{AnchorDeserialize, AnchorSerialize};

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
