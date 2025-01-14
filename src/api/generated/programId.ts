import { PublicKey } from "@solana/web3.js";

// Program ID defined in the provided IDL. Do not edit, it will get overwritten.
export const PROGRAM_ID_IDL = new PublicKey(
  "7uXnX9gW2smtc5wvaeU3hBQBViRFpyD6C6D5rTb5J16E"
);

// Program ID passed with the cli --program-id flag when running the code generator. Do not edit, it will get overwritten.
export const PROGRAM_ID_CLI = new PublicKey(
  "7uXnX9gW2smtc5wvaeU3hBQBViRFpyD6C6D5rTb5J16E"
);

// This constant will not get overwritten on subsequent code generations and it's safe to modify it's value.
export const PROGRAM_ID: PublicKey = PROGRAM_ID_CLI;
