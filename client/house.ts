// @ts-nocheck

import * as anchor from "@coral-xyz/anchor";
import fs from 'fs'
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  AuthorityType,
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  createMintToInstruction,
  getMint,
  setAuthority,
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Keypair, PublicKey, TransactionInstruction } from "@solana/web3.js";
import { sleep } from "@switchboard-xyz/common";
import {
  Mint,
  QueueAccount,
  SwitchboardProgram,
  TransactionObject,
} from "@switchboard-xyz/solana.js";
import { HouseState, HouseStateJSON } from "./generated/accounts/index";
import { houseInit } from "./generated/instructions/index";

export class HouseAccountDoesNotExist extends Error {
  readonly name = "HouseAccountDoesNotExist";
  readonly msg = "Failed to fetch the HouseState account.";

  constructor() {
    super("HouseAccountDoesNotExist: Failed to fetch the HouseState account.");
  }
}

export interface HouseJSON extends HouseStateJSON {
  publicKey: string;
}

export class House {
  state: HouseState;

  constructor(
    readonly program: anchor.Program,
    readonly publicKey: PublicKey,
    state: HouseState
  ) {
    this.state = state;
  }

  static fromSeeds(programId: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("HOUSESEED")],
      programId
    );
  }

  async reload(): Promise<void> {
    const newState = await HouseState.fetch(
     this.program.provider.connection,
       
      this.publicKey
    );
    if (newState === null) {
      throw new Error(`Failed to fetch the new House account state`);
    }
    this.state = newState;
  }

  toJSON(): HouseJSON {
    return {
      publicKey: this.publicKey.toString(),
      ...this.state.toJSON(),
    };
  }

  getQueueAccount(switchboardProgram: SwitchboardProgram): QueueAccount {
    const queueAccount = new QueueAccount(
      switchboardProgram,
      this.state.switchboardQueue
    );
    return queueAccount;
  }

  static async create(
    program: anchor.Program,
    switchboardQueue: QueueAccount,
    mint: Keypair = anchor.web3.Keypair.generate()
  ): Promise<House> {
    const [initHouse, houseKey] = await House.createReq(
      program,
      switchboardQueue,
      mint
    );

    const signature = await switchboardQueue.program.signAndSend(initHouse, {
      skipPreflight: true,
    });

    let retryCount = 5;
    while (retryCount) {
      const houseState = await HouseState.fetch(
         program.provider.connection,
       
        houseKey
      );
      if (houseState !== null) {
        return new House(program, houseKey, houseState);
      }
      await sleep(1000);
      --retryCount;
    }

    throw new Error(`Failed to create new HouseAccount`);
  }

  static async createReq(
    program: anchor.Program,
    switchboardQueue: QueueAccount,
    mint: Keypair = anchor.web3.Keypair.generate()
  ): Promise<[TransactionObject, PublicKey]> {
    const payer = switchboardQueue.program.walletPubkey;

    const [houseKey, houseBump] = House.fromSeeds(program.programId);

    const mintPubkey: PublicKey = mint.publicKey;

    const [tokenVault] = PublicKey.findProgramAddressSync(
      [houseKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintPubkey.toBuffer()],
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    
    const initHouse = houseInit(
     
      {  },
      {
        house: houseKey,
        authority: payer,
        switchboardMint: switchboardQueue.program.mint.address,
        switchboardQueue: switchboardQueue.publicKey,
        mint: mintPubkey,
        houseVault: tokenVault,
        payer: payer,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenProgram2022: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      {
        programId: program.programId
      },
    );

console.log((payer.toBase58(),mint.publicKey.toBase58(), houseKey.toBase58()))
console.log(initHouse)
    return [new TransactionObject(payer, [initHouse], [mint]), houseKey];
  }

  static async load(program: anchor.Program): Promise<House> {
    const connection = program.provider.connection;
    const [houseKey, houseBump] = House.fromSeeds(program.programId);

    let houseState = await HouseState.fetch(
      program.provider.connection,
      houseKey
    );
    if (houseState !== null) {
      return new House(program, houseKey, houseState);
    }

    throw new Error(`House account has not been created yet`);
  }

  static async getOrCreate(
    program: anchor.Program,
    switchboardQueue?: QueueAccount,
    mint?: Keypair
  ): Promise<House> {
    try {
      const house = await House.load(program);
      return house;
    } catch (error: any) {
      if (
        !error.toString().includes("House account has not been created yet")
      ) {
        throw error;
      }
    }

    if (!switchboardQueue) {
      throw new Error(
        `Need to provide Switchboard queueAccount if the House hasnt been initialized yet`
      );
    }

    return House.create(program, switchboardQueue, mint ?? Keypair.generate());
  }

  async loadMint(): Promise<Mint> {
    const mint = await Mint.load(
      this.program.provider as anchor.AnchorProvider,
      this.state.mint
    );
    return mint;
  }
}
