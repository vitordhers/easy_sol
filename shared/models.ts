import { Buffer } from "node:buffer";
import {
  AbstractType,
  Constructor,
  deserialize,
  serialize,
} from "npm:@dao-xyz/borsh";

export abstract class SerializationHelper {
  private constructor() {
    throw new Error("StupidMath is a static class and cannot be instantiated!");
  }
  static serialize<T>(schema: T): Buffer {
    return Buffer.from(serialize(schema));
  }

  static getDataSize<T>(schema: T): number {
    return SerializationHelper.serialize(schema).length;
  }

  static deserialize<T>(
    buffer: Buffer,
    ctor: Constructor<T> | AbstractType<T>,
  ): T {
    return deserialize(buffer, ctor);
  }
}
