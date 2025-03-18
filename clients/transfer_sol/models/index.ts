import { field } from "npm:@dao-xyz/borsh";
import { SerializableSchema } from "../../../shared/models.ts";

export class TransferSchema extends SerializableSchema {
  @field({ type: "u64" })
  value: bigint;

  constructor(value: bigint) {
    super();
    this.value = value;
  }
}
