import { serialize } from "npm:@dao-xyz/borsh";
import { CalculatorInstruction, CalculatorSchema } from "../models/index.ts";

export const CALCULATOR_SIZE = serialize(new CalculatorSchema()).length;

export const CALCULATOR_INSTRUCTIONS_SIZE = serialize(
  new CalculatorInstruction(),
).length;
