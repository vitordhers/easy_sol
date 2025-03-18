import { serialize } from "npm:@dao-xyz/borsh";

export abstract class SerializableSchema {
  serialize() {
    return serialize(this);
  }
}
