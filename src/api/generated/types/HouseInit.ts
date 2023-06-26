import { FlipProgram } from "../../program";
import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh";

export interface HouseInitFields {
  house: PublicKey;
  authority: PublicKey;
  switchboardMint: PublicKey;
  switchboardQueue: PublicKey;
  mint: PublicKey;
  houseVault: PublicKey;
  payer: PublicKey;
  systemProgram: PublicKey;
  tokenProgram: PublicKey;
  tokenProgram2022: PublicKey;
  associatedTokenProgram: PublicKey;
  rent: PublicKey;
}

export interface HouseInitJSON {
  house: string;
  authority: string;
  switchboardMint: string;
  switchboardQueue: string;
  mint: string;
  houseVault: string;
  payer: string;
  systemProgram: string;
  tokenProgram: string;
  tokenProgram2022: string;
  associatedTokenProgram: string;
  rent: string;
}

export class HouseInit {
  readonly house: PublicKey;
  readonly authority: PublicKey;
  readonly switchboardMint: PublicKey;
  readonly switchboardQueue: PublicKey;
  readonly mint: PublicKey;
  readonly houseVault: PublicKey;
  readonly payer: PublicKey;
  readonly systemProgram: PublicKey;
  readonly tokenProgram: PublicKey;
  readonly tokenProgram2022: PublicKey;
  readonly associatedTokenProgram: PublicKey;
  readonly rent: PublicKey;

  constructor(fields: HouseInitFields) {
    this.house = fields.house;
    this.authority = fields.authority;
    this.switchboardMint = fields.switchboardMint;
    this.switchboardQueue = fields.switchboardQueue;
    this.mint = fields.mint;
    this.houseVault = fields.houseVault;
    this.payer = fields.payer;
    this.systemProgram = fields.systemProgram;
    this.tokenProgram = fields.tokenProgram;
    this.tokenProgram2022 = fields.tokenProgram2022;
    this.associatedTokenProgram = fields.associatedTokenProgram;
    this.rent = fields.rent;
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("house"),
        borsh.publicKey("authority"),
        borsh.publicKey("switchboardMint"),
        borsh.publicKey("switchboardQueue"),
        borsh.publicKey("mint"),
        borsh.publicKey("houseVault"),
        borsh.publicKey("payer"),
        borsh.publicKey("systemProgram"),
        borsh.publicKey("tokenProgram"),
        borsh.publicKey("tokenProgram2022"),
        borsh.publicKey("associatedTokenProgram"),
        borsh.publicKey("rent"),
      ],
      property
    );
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new HouseInit({
      house: obj.house,
      authority: obj.authority,
      switchboardMint: obj.switchboardMint,
      switchboardQueue: obj.switchboardQueue,
      mint: obj.mint,
      houseVault: obj.houseVault,
      payer: obj.payer,
      systemProgram: obj.systemProgram,
      tokenProgram: obj.tokenProgram,
      tokenProgram2022: obj.tokenProgram2022,
      associatedTokenProgram: obj.associatedTokenProgram,
      rent: obj.rent,
    });
  }

  static toEncodable(fields: HouseInitFields) {
    return {
      house: fields.house,
      authority: fields.authority,
      switchboardMint: fields.switchboardMint,
      switchboardQueue: fields.switchboardQueue,
      mint: fields.mint,
      houseVault: fields.houseVault,
      payer: fields.payer,
      systemProgram: fields.systemProgram,
      tokenProgram: fields.tokenProgram,
      tokenProgram2022: fields.tokenProgram2022,
      associatedTokenProgram: fields.associatedTokenProgram,
      rent: fields.rent,
    };
  }

  toJSON(): HouseInitJSON {
    return {
      house: this.house.toString(),
      authority: this.authority.toString(),
      switchboardMint: this.switchboardMint.toString(),
      switchboardQueue: this.switchboardQueue.toString(),
      mint: this.mint.toString(),
      houseVault: this.houseVault.toString(),
      payer: this.payer.toString(),
      systemProgram: this.systemProgram.toString(),
      tokenProgram: this.tokenProgram.toString(),
      tokenProgram2022: this.tokenProgram2022.toString(),
      associatedTokenProgram: this.associatedTokenProgram.toString(),
      rent: this.rent.toString(),
    };
  }

  static fromJSON(obj: HouseInitJSON): HouseInit {
    return new HouseInit({
      house: new PublicKey(obj.house),
      authority: new PublicKey(obj.authority),
      switchboardMint: new PublicKey(obj.switchboardMint),
      switchboardQueue: new PublicKey(obj.switchboardQueue),
      mint: new PublicKey(obj.mint),
      houseVault: new PublicKey(obj.houseVault),
      payer: new PublicKey(obj.payer),
      systemProgram: new PublicKey(obj.systemProgram),
      tokenProgram: new PublicKey(obj.tokenProgram),
      tokenProgram2022: new PublicKey(obj.tokenProgram2022),
      associatedTokenProgram: new PublicKey(obj.associatedTokenProgram),
      rent: new PublicKey(obj.rent),
    });
  }

  toEncodable() {
    return HouseInit.toEncodable(this);
  }
}
