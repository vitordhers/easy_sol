import { field } from "npm:@dao-xyz/borsh";

export class TransferSchema {
  @field({ type: "u64" })
  value: bigint;

  constructor(value?: bigint) {
    this.value = value || 0n;
  }
}
