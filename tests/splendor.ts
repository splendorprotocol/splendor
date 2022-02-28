const assert = require("assert");
const fs = require('fs');
import { Keypair } from '@solana/web3.js'
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Splendor } from "../target/types/splendor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const TOKEN_A_MINT = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const TOKEN_B_MINT = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
const TUTOKEN_A_MINT = "Amig8TisuLpzun8XyGfC5HJHHGUQEscjLgoTWsCCKihg";
const TUTOKEN_B_MINT = "gLhY2arqFpmVGkpbBbTi3TeWbsWevA8dqrwbKacK3vJ";


describe("splendor", () =>  {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env()
  anchor.setProvider(provider);


  const program = anchor.workspace.Splendor as Program<Splendor>;

  // Generate keypair for vaultAdmin
  const vaultAdmin = anchor.web3.Keypair.fromSecretKey(new Uint8Array([163,72,84,87,235,8,243,88,220,9,114,139,248,126,32,87,184,27,39,63,126,80,77,194,107,36,159,41,242,220,57,31,7,67,143,30,162,22,245,203,124,227,92,15,133,20,96,35,0,85,161,213,94,67,152,247,156,155,235,150,253,61,14,51]));
  const spUSDkeypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array([94,72,178,199,151,81,193,85,53,212,142,34,37,128,124,29,79,234,16,198,97,109,182,160,112,20,110,35,144,56,23,232,13,5,66,63,76,188,59,12,250,43,142,85,124,197,238,48,27,80,40,211,214,98,248,234,64,110,121,137,27,19,69,3]));
  const redeemableMint = spUSDkeypair;
  const vaultName = "USDC-USDT";

  let programConstants = Object.assign({}, ...program.idl.constants.map((x) => ({[x.name]: x.value.slice(1,-1)})));

  // const tokenAMint = anchor.web3.Keypair.generate();
  // const tokenBMint = anchor.web3.Keypair.generate();
  // const tutokenAMint = anchor.web3.Keypair.generate();
  // const tutokenBMint = anchor.web3.Keypair.generate();
  const tokenAMint = TOKEN_A_MINT;
  const tokenBMint = TOKEN_B_MINT;
  const tutokenAMint = TUTOKEN_A_MINT;
  const tutokenBMint = TUTOKEN_B_MINT;

  const vaultBumps = anchor.web3.Keypair.generate();

  // prepare PDA variables
  // let vaultInfo;
  // let vaultAuthority;
  // let vaultTokenA;
  // let vaultTokenB;
  // let vaultTutokenA
  // let vaultTutokenB
  // let authorityBump;
  // let infoBump;
  // let tokenABump;
  // let tokenBBump;
  // let tutokenABump;
  // let tutokenBBump;

  let systemProgram = anchor.web3.SystemProgram;
  //let info_bump, admin_bump, token_a_bump, token_b_bump, tutoken_a_bump, tutoken_b_bump = [255,255,255,255,255,255];

  it("Vault initialized!", async () => {
    // Gather PDAs
    // vaultInfo
    let [vaultInfo, infoBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode(programConstants['VAULT_INFO_SEED'])),
        Buffer.from(anchor.utils.bytes.utf8.encode(vaultName))
      ],
      program.programId
    )
    console.log("Found Info PDA (seed =", programConstants['VAULT_INFO_SEED'], "):", vaultInfo.toString(), infoBump);
    // vaultAuthority
    let [vaultAuthority, authorityBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode(programConstants['VAULT_AUTHORITY_SEED'])),
        Buffer.from(anchor.utils.bytes.utf8.encode(vaultName))
      ],
      program.programId
    )
    console.log("Found Authority PDA (seed =", programConstants["VAULT_AUTHORITY_SEED"], "):", vaultAuthority.toString(), authorityBump);
    // vaultTokenA
    let [vaultTokenA, tokenABump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode(programConstants['VAULT_TOKENA_SEED'])),
        Buffer.from(anchor.utils.bytes.utf8.encode(vaultName))
      ],
      program.programId
    )
    console.log("Found TokenA ATA (seed = ", programConstants["VAULT_TOKENA_SEED"], "):", vaultTokenA.toString(), tokenABump);
    // vaultTokenB
    let [vaultTokenB, tokenBBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode(programConstants['VAULT_TOKENB_SEED'])),
        Buffer.from(anchor.utils.bytes.utf8.encode(vaultName))
      ],
      program.programId
    )
    console.log("Found TokenB ATA (seed = ", programConstants["VAULT_TOKENB_SEED"], "):", vaultTokenB.toString(), tokenBBump);
    // vaultTutokena
    let [vaultTutokenA, tutokenABump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode(programConstants['VAULT_TUTOKENA_SEED'])),
        Buffer.from(anchor.utils.bytes.utf8.encode(vaultName))
      ],
      program.programId
    )
    console.log("Found TutokenA ATA (seed =", programConstants['VAULT_TUTOKENA_SEED'], "):", vaultTutokenA.toString(), tutokenABump);
    // vaultTutokenB
    let [vaultTutokenB, tutokenBBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode(programConstants['VAULT_TUTOKENB_SEED'])),
        Buffer.from(anchor.utils.bytes.utf8.encode(vaultName))
      ],
      program.programId
    )
    console.log("Found TutokenB ATA (seed =", programConstants['VAULT_TUTOKENB_SEED'], "):", vaultTutokenB.toString(), tutokenBBump);
    // redeemableMint
    // let [redeemableMint, redeemBump] = await anchor.web3.PublicKey.findProgramAddress(
    //   [
    //     Buffer.from(anchor.utils.bytes.utf8.encode(programConstants['VAULT_REDEEMABLE_MINT_SEED'])),
    //     Buffer.from(anchor.utils.bytes.utf8.encode(vaultName))
    //   ],
    //   program.programId
    // )
    //console.log("Found RedeemableMint PDA(seed = ", programConstants["VAULT_REDEEMABLE_MINT_SEED"], "):", redeemableMint.toString(), redeemBump);
    console.log("Redeemable Mint (grinded) =", redeemableMint.publicKey.toString());
    console.log("-".repeat(50));
    
    console.log("Admin Key:", vaultAdmin.publicKey.toString());
    console.log("Info Key:", vaultInfo.toString());
    console.log("Authority Key:", vaultAuthority.toString());
    console.log("TokenA Key:", vaultTokenA.toString());
    console.log("TokenB Key:", vaultTokenB.toString());
    console.log("TutokenA Key:", vaultTutokenA.toString());
    console.log("TutokenB Key:", vaultTutokenB.toString());
    console.log("redeemableMint Key:", redeemableMint.publicKey.toString());
    console.log("SystemProgram Key:", systemProgram.programId.toString());
    console.log("TokenProgram Key:", TOKEN_PROGRAM_ID.toString());
    console.log("AssociatedTokenProgram Key:", ASSOCIATED_TOKEN_PROGRAM_ID.toString());
    
    const tx = await program.rpc.initializeVault(
      vaultName, 
      // infoBump,
      // authorityBump,
      // tokenABump,
      // tokenBBump,
      // tutokenABump,
      // tutokenBBump,
      // redeemBump,
      {
      accounts : {
        // Vault Stuff
        vaultAdmin: vaultAdmin.publicKey,
        vaultInfo: vaultInfo,
        vaultAuthority: vaultAuthority,
        // Token Mints
        tokenAMint: tokenAMint,
        tokenBMint: tokenBMint,
        tutokenAMint: tutokenAMint,
        tutokenBMint: tutokenBMint,
        // Token Vaults
        vaultTokenA: vaultTokenA,
        vaultTokenB: vaultTokenB,
        vaultTutokenA: vaultTutokenA,
        vaultTutokenB: vaultTutokenB,
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
        redeemableMint
      ]
    });
    console.log("Your transaction signature", tx);

    //let vaultAccount = await program.account.vault_info.fetch(vault_info.publicKey);
    //let vaultAccount = await program.account.vaultBumps.fetch(vaultBumps.publicKey)
  })
});
