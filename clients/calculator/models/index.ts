import { deserialize, field, serialize } from "npm:@dao-xyz/borsh";
import { Operation } from "../enums/index.ts";
import { Buffer } from "node:buffer";

export class CalculatorSchema {
  @field({ type: "f32" })
  value: number = 0;

  constructor() {}

  static deserialize(buffer: Buffer) {
    return deserialize(buffer, CalculatorSchema);
  }
}

export class OperationSchema {
  @field({ type: "u8" })
  value: Operation;

  constructor(value?: Operation) {
    this.value = value || Operation.Add;
  }
}

export class CalculatorInstruction {
  @field({ type: OperationSchema })
  operation: OperationSchema;

  @field({ type: "f32" })
  operating_value: number;

  constructor(operation?: Operation, value?: number) {
    this.operation = new OperationSchema(operation);
    this.operating_value = value || 0;
  }

  serialize() {
    return serialize(this);
  }
}
