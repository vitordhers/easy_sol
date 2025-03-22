import { field } from "npm:@dao-xyz/borsh";
import { Operation } from "../enums/index.ts";

export class CalculatorSchema {
  @field({ type: "f32" })
  value: number;

  constructor(value?: number) {
    this.value = value || 0;
  }
}

export class OperationSchema {
  @field({ type: "u8" })
  value: Operation;

  constructor(value?: Operation) {
    this.value = value || Operation.Add;
  }
}

export class CalculatorInstructionSchema {
  @field({ type: OperationSchema })
  operation: OperationSchema;

  @field({ type: "f32" })
  operating_value: number;

  constructor(operation?: Operation, value?: number) {
    this.operation = new OperationSchema(operation);
    this.operating_value = value || 0;
  }
}
