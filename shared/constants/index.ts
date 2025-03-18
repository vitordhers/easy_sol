import { resolve } from "@std/path";

export const CONFIG_FILE_PATH = resolve(
  Deno.env.get("HOME") || "~",
  ".config",
  "solana",
  "cli",
  "config.yml",
);

export const PROGRAMS_PATH = resolve("contracts", "target", "deploy");

export const WALLETS_PATH = resolve("wallets");
