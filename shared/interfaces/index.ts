import { Commitment, PublicKey } from "npm:@solana/web3.js";

export interface CliConfig {
  json_rpc_url: string;
  websocket_url: string;
  keypair_path: string;
  address_labels: Record<string, any>;
  commitment: Commitment;
}

export interface PDA {
  pda: PublicKey;
  bump: number;
}
