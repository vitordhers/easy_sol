import { field, variant } from "npm:@dao-xyz/borsh";
import { SerializableSchema } from "../../../shared/models.ts";

export abstract class Super extends SerializableSchema {}

@variant(0)
export class FungibleTokenData extends Super {
  @field({ type: "u8" })
  decimals: number;

  @field({ type: "u64" })
  initial_supply: bigint;

  @field({ type: "bool" })
  should_freeze_after_mint: boolean;

  constructor(
    decimals: number,
    initialSupply: bigint,
    shouldFreezeAfterMint: boolean,
  ) {
    super();
    this.decimals = decimals;
    this.initial_supply = initialSupply;
    this.should_freeze_after_mint = shouldFreezeAfterMint;
  }
}

@variant(1)
export class FungibleAssetData extends Super {
  @field({ type: "u8" })
  decimals: number;

  @field({ type: "u64" })
  quantity: bigint;

  constructor(decimals: number, quantity: bigint) {
    super();
    this.decimals = decimals;
    this.quantity = quantity;
  }
}

@variant(2)
export class NonFungibleTokenData extends Super {
  constructor() {
    super();
  }
}
