import { resolve } from "@std/path";
import { PublicKey } from "npm:@solana/web3.js";

export const CONFIG_FILE_PATH = resolve(
  Deno.env.get("HOME") || "~",
  ".config",
  "solana",
  "cli",
  "config.yml",
);

export const PROGRAMS_PATH = resolve("contracts", "target", "deploy");

export const WALLETS_PATH = resolve("wallets");

export const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
);
