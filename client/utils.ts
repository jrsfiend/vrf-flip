import * as anchor from "@project-serum/anchor";
import * as anchor24 from "anchor-24-2";
import * as spl from "@solana/spl-token-v2";
import {
  Cluster,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import * as sbv2 from "@switchboard-xyz/switchboard-v2";
import { PROGRAM_ID_CLI } from "./generated/programId";
import { FlipProgram } from "./types";
import { User } from "./user";
import Big from "big.js";

const DEFAULT_COMMITMENT = "confirmed";

export const defaultRpcForCluster = (cluster: Cluster | "localnet") => {
  switch (cluster) {
    case "mainnet-beta":
      return "https://ssc-dao.genesysgo.net";
    case "devnet":
      return "https://devnet.genesysgo.net";
    case "localnet":
      return "http://localhost:8899";
    default:
      throw new Error(`Failed to find RPC_URL for cluster ${cluster}`);
  }
};

export interface FlipUser {
  keypair: anchor.web3.Keypair;
  switchboardProgram: anchor24.Program;
  switchTokenWallet: anchor.web3.PublicKey;
  user: User;
}

export async function getFlipProgram(
  rpcEndpoint: string
): Promise<FlipProgram> {
  const programId = new PublicKey(PROGRAM_ID_CLI);
  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection(rpcEndpoint, { commitment: DEFAULT_COMMITMENT }),
    new sbv2.AnchorWallet(Keypair.generate()),
    { commitment: DEFAULT_COMMITMENT }
  );

  const idl = await anchor.Program.fetchIdl(programId, provider);
  if (!idl)
    throw new Error(
      `Failed to find IDL for program [ ${programId.toBase58()} ]`
    );

  return new anchor.Program(
    idl,
    programId,
    provider,
    new anchor.BorshCoder(idl)
  );
}

export async function createFlipUser(
  program: FlipProgram,
  queueAccount: sbv2.OracleQueueAccount,
  wSolAmount = 0.2
): Promise<FlipUser> {
  const switchboardProgram = queueAccount.program;

  const keypair = anchor.web3.Keypair.generate();
  const airdropTxn = await program.provider.connection.requestAirdrop(
    keypair.publicKey,
    1 * anchor.web3.LAMPORTS_PER_SOL
  );
  await program.provider.connection.confirmTransaction(airdropTxn);

  const provider = new anchor.AnchorProvider(
    switchboardProgram.provider.connection,
    new sbv2.AnchorWallet(keypair),
    {}
  );
  const flipProgram = new anchor.Program(
    program.idl,
    program.programId,
    provider
  );
  const newSwitchboardProgram = new anchor24.Program(
    switchboardProgram.idl,
    switchboardProgram.programId,
    provider
  );
  const switchTokenWallet = await spl.createWrappedNativeAccount(
    newSwitchboardProgram.provider.connection,
    keypair,
    keypair.publicKey,
    wSolAmount * anchor.web3.LAMPORTS_PER_SOL
  );

  const user = await User.create(flipProgram, newSwitchboardProgram);

  return {
    keypair,
    switchboardProgram: newSwitchboardProgram,
    switchTokenWallet,
    user,
  };
}

export const tokenAmountToBig = (tokenAmount: anchor.BN, decimals = 9): Big => {
  const bigTokenAmount = new Big(tokenAmount.toString(10));

  const denominator = new Big(10).pow(decimals);
  const oldDp = Big.DP;
  Big.DP = 20;
  const result = bigTokenAmount.div(denominator);
  Big.DP = oldDp;
  return result;
};

export const verifyPayerBalance = async (
  connection: Connection,
  payer: PublicKey,
  minAmount = 0.1 * LAMPORTS_PER_SOL,
  currentBalance?: number
): Promise<void> => {
  const payerBalance = currentBalance ?? (await connection.getBalance(payer));
  if (payerBalance > minAmount) {
    console.log(
      `Payer has sufficient funds, ${payerBalance / LAMPORTS_PER_SOL} > ${
        minAmount / LAMPORTS_PER_SOL
      }`
    );
    return;
  }

  try {
    const airdropTxn = await connection.requestAirdrop(
      payer,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropTxn);
  } catch (error) {
    console.log(`Failed to request an airdrop`);
    console.error(error);
  }
};