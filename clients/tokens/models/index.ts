import { field, option, variant, vec } from "npm:@dao-xyz/borsh";

export class FungibleTokenMetadata {
  @field({ type: "string" })
  name: string;
  @field({ type: "string" })
  symbol: string;
  @field({ type: "string" })
  uri: string;

  constructor(name: string, symbol: string, uri: string) {
    this.name = name;
    this.symbol = symbol;
    this.uri = uri;
  }
}

export class FungibleAssetMetadata {
  // HEADS UP: order of similar type properties is of utmost importance!!!
  // check index property of field decorator param for managing order accondingly, if needed!
  @field({ type: "string" })
  name: string;
  @field({ type: "string" })
  symbol: string;
  @field({ type: "string" })
  uri: string;
  @field({ type: "u64" })
  uses: bigint;

  constructor(name: string, symbol: string, uri: string, uses: bigint) {
    this.name = name;
    this.symbol = symbol;
    this.uri = uri;
    this.uses = uses;
  }
}

export class NonFungibleTokenMetadata {
  @field({ type: "string" })
  name: string;
  @field({ type: "string" })
  symbol: string;
  @field({ type: "string" })
  uri: string;
  @field({ type: "u16" })
  seller_fee_basis_points: number;
  @field({ type: option(vec("string")) })
  creators_addresses?: string[];
  @field({ type: option("string") })
  collection_address?: string;

  constructor(
    name: string,
    symbol: string,
    uri: string,
    seller_fee_basis_points: number,
    creators_addresses?: string[],
    collection_address?: string,
  ) {
    this.name = name;
    this.symbol = symbol;
    this.uri = uri;
    this.seller_fee_basis_points = seller_fee_basis_points;
    this.creators_addresses = creators_addresses;
    this.collection_address = collection_address;
  }
}

export abstract class TokenData {}

@variant(0)
export class FungibleTokenData extends TokenData {
  @field({ type: "u8" })
  decimals: number;
  @field({ type: "u64" })
  initial_supply: bigint;
  @field({ type: "bool" })
  should_freeze_after_mint: boolean;
  @field({ type: FungibleTokenMetadata })
  metadata: FungibleTokenMetadata;

  constructor(
    decimals: number,
    initialSupply: bigint,
    shouldFreezeAfterMint: boolean,
    metadata: FungibleTokenMetadata,
  ) {
    super();
    this.decimals = decimals;
    this.initial_supply = initialSupply;
    this.should_freeze_after_mint = shouldFreezeAfterMint;
    this.metadata = metadata;
  }
}

@variant(1)
export class FungibleAssetData extends TokenData {
  @field({ type: "u8" })
  decimals: number;
  @field({ type: "u64" })
  quantity: bigint;
  @field({ type: FungibleAssetMetadata })
  metadata: FungibleAssetMetadata;
  constructor(
    decimals: number,
    quantity: bigint,
    metadata: FungibleAssetMetadata,
  ) {
    super();
    this.decimals = decimals;
    this.quantity = quantity;
    this.metadata = metadata;
  }
}

@variant(2)
export class NonFungibleTokenData extends TokenData {
  @field({ type: NonFungibleTokenMetadata })
  metadata: NonFungibleTokenMetadata;

  constructor(metadata: NonFungibleTokenMetadata) {
    super();
    this.metadata = metadata;
  }
}
