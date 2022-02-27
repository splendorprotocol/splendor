const assert = require("assert");
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Splendor } from "../target/types/splendor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("splendor", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Splendor as Program<Splendor>;

  // Generate keypair for pool
  const vaultAdmin = anchor.web3.Keypair.generate();
  const vaultInfo = anchor.web3.Keypair.generate();
  const vaultAuthority = anchor.web3.Keypair.generate();
  const tokenAMint = anchor.web3.Keypair.generate();
  const tokenBMint = anchor.web3.Keypair.generate();
  const tutokenAMint = anchor.web3.Keypair.generate();
  const tutokenBMint = anchor.web3.Keypair.generate();
  const vaultTokenA = anchor.web3.Keypair.generate();
  const vaultTokenB = anchor.web3.Keypair.generate();
  const vaultTutokenA = anchor.web3.Keypair.generate();
  const vaultTutokenB = anchor.web3.Keypair.generate();
  const vaultBumps = anchor.web3.Keypair.generate();
  const redeemableMint = anchor.web3.Keypair.generate();

  let systemProgram = anchor.web3.SystemProgram;
  let bumps: Array<number>;
  let info_bump, admin_bump, token_a_bump, token_b_bump, tutoken_a_bump, tutoken_b_bump = [255,255,255,255,255,255];
  it("Vault initialized!", async () => {


    console.log("Admin Key:", vaultAdmin.publicKey.toString());
    console.log("Info Key:", vaultInfo.publicKey.toString());
    console.log("Authority Key:", vaultAuthority.publicKey.toString());
    console.log("TokenA Key:", vaultTokenA.publicKey.toString());
    console.log("TokenB Key:", vaultTokenB.publicKey.toString());
    console.log("TutokenA Key:", vaultTutokenA.publicKey.toString());
    console.log("TutokenB Key:", vaultTutokenB.publicKey.toString());
    console.log("redeemableMint Key:", redeemableMint.publicKey.toString());
    // Add your test here.
    const tx = await program.rpc.initializeVault(
      "my_vault", 
      info_bump,
      admin_bump,
      token_a_bump,
      token_b_bump,
      tutoken_a_bump,
      tutoken_b_bump,
      {
      accounts : {
        // Vault Stuff
        vaultAdmin: vaultAdmin.publicKey,
        vaultInfo: vaultInfo.publicKey,
        vaultAuthority: vaultAuthority.publicKey,
        // Token Mints
        tokenAMint: tokenAMint.publicKey,
        tokenBMint: tokenBMint.publicKey,
        tutokenAMint: tutokenAMint.publicKey,
        tutokenBMint: tutokenBMint.publicKey,
        // Token Vaults
        vaultTokenA: vaultTokenA.publicKey,
        vaultTokenB: vaultTokenB.publicKey,
        vaultTutokenA: vaultTutokenB.publicKey,
        vaultTutokenB: vaultTutokenB.publicKey,
        // spX Mint
        redeemableMint: redeemableMint.publicKey,
        // System Stuff
        systemProgram: systemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      },
      signers : [
        vaultAdmin, 
        //vaultInfo, 
        //vaultAuthority, 
        //vaultTokenA, 
        //vaultTokenB, 
        //vaultTutokenA, 
        //vaultTutokenB, 
        //redeemableMint
      ]
    });
    console.log("Your transaction signature", tx);


    //let vaultAccount = await program.account.vault_info.fetch(vault_info.publicKey);
    //let vaultAccount = await program.account.vaultBumps.fetch(vaultBumps.publicKey)
  })
});
