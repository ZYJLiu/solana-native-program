// npm init -y
// npm install --save-dev typescript ts-node @types/node
// create tsconfig.json
// npx ts-node --esm src/index.ts
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as borsh from "borsh";

const PROGRAM_ID = new PublicKey(
  "HPV7ABfgTwFfvcpxa6qRE9dgzhtSL58Y62yrDe5ntnx5"
);

// Define the account data structure
class AccountData {
  data: number;
  constructor(fields: { data: number } | undefined = undefined) {
    this.data = fields?.data ?? 0;
  }
}

// Define the schema for Borsh serialization/deserialization
const AccountDataSchema: borsh.Schema = {
  struct: { data: "u64" },
};

// Connect to the Solana cluster
const connection = new Connection("http://127.0.0.1:8899", "confirmed");

// Generate a new keypair for the account that will be created
const newAccountKeypair = Keypair.generate();

// Use your wallet keypair here
const payerKeypair = Keypair.generate(); // Replace with your actual keypair

// Airdrop some SOL to the payer for testing on devnet
const airdropSignature = await connection.requestAirdrop(
  payerKeypair.publicKey,
  LAMPORTS_PER_SOL // 1 SOL
);
await connection.confirmTransaction(airdropSignature);

// Prepare the instruction data
const instructionData = Buffer.alloc(9);
instructionData.writeUInt8(0, 0); // Instruction index for Initialize
instructionData.writeBigUInt64LE(BigInt(42), 1); // The data to be stored

// Create the instruction
const instruction = new TransactionInstruction({
  keys: [
    { pubkey: newAccountKeypair.publicKey, isSigner: true, isWritable: true },
    { pubkey: payerKeypair.publicKey, isSigner: true, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ],
  programId: PROGRAM_ID,
  data: instructionData,
});

// Create and send the transaction
const transaction = new Transaction().add(instruction);
const signature = await sendAndConfirmTransaction(connection, transaction, [
  payerKeypair,
  newAccountKeypair,
]);

console.log(`Transaction signature: ${signature}`);

// Fetch the created account
const accountInfo = await connection.getAccountInfo(
  newAccountKeypair.publicKey
);

if (accountInfo === null) {
  throw new Error("Account not found");
}

// Deserialize the account data
const deserializedAccountData = borsh.deserialize(
  AccountDataSchema,
  accountInfo.data
) as { data: number };

console.log(`Account data: ${deserializedAccountData.data}`);
