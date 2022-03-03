const assert = require("assert");
const fs = require('fs');
import { Keypair, PublicKey} from '@solana/web3.js'
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Splendor } from "../target/types/splendor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { rpc } from '@project-serum/anchor/dist/cjs/utils';

const TOKEN_A_MINT = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"; //USDC
const TOKEN_B_MINT = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"; //USDT
const TUTOKEN_A_MINT = "Amig8TisuLpzun8XyGfC5HJHHGUQEscjLgoTWsCCKihg";
const TUTOKEN_B_MINT = "gLhY2arqFpmVGkpbBbTi3TeWbsWevA8dqrwbKacK3vJ";

const debug = true;
if (!debug) {
  console.log = function(){};
}

describe("splendor", () =>  {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env()
  anchor.setProvider(provider);


  const program = anchor.workspace.Splendor as Program<Splendor>;

  // Generate keypair for vaultAdmin
  const data = fs.readFileSync('devlet2.json', {encoding:'utf8', flag:'r'}).slice(1,-1).split(",");
  const redeemData = fs.readFileSync('vanity/USD6kRczLP5uV5G9dDSRFRBgnvJR9Po6q1V1vSw1H4q.json', {encoding:'utf8', flag:'r'}).slice(1,-1).split(",");
  const vaultAdmin = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(data));
  const spUSDkeypair = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(redeemData));
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
        redeemableMint
      ]
    });
    console.log("Your transaction signature", tx);

    // Fetch vaultInfo account
    let vaultInfoAccount = await program.account.vaultInfo.fetch(vaultInfo);
    // Assert vaultName stored properly
    assert(Buffer.from(Uint8Array.from(vaultInfoAccount.vaultName.filter(x => x != 0 ))).equals(Buffer.from(anchor.utils.bytes.utf8.encode(vaultName))));
  })

  it("User Deposit!", async () => {

    const user = vaultAdmin;

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


    // const usdcMintAccount = new Token(
    //   provider.connection,
    //   new anchor.web3.PublicKey(tutokenAMint),
    //   TOKEN_PROGRAM_ID,
    //   (provider.wallet as anchor.Wallet).payer
    // );

    // const userUSDC = await usdcMintAccount.createAssociatedTokenAccount(
    //   vaultAdmin.publicKey
    // );
    const userUSDC = new PublicKey("DpxS9YTv7e7MD9USgniaG4G5n4gVGAk8HJ6p7AXDXcxq");
    const userUSDT = new PublicKey("5KuYEobaCTkQntwdHBZZJpo8HLAb3ruvcWKC1z5RHGJk");
    const userTUUSDC = new PublicKey("6wQkK76HdRLtVB11V6Tcvp92WmSwSSf6V64kvYbe3xTd");
    const userTUUSDT = new PublicKey("34bmioDCWJqaig6SN3b28TAAtrbWL35byabBVmagVzR7");

    console.log("after create usdc ata", userUSDC.toString())
    //console.log("user public key", user.publicKey.toString())

    const tx = await program.rpc.deposit(
      
      // Instruction Arguments
      [infoBump, authorityBump, tokenABump, tokenBBump, tutokenABump, tutokenBBump],
      1, 
      1,
      // Accounts
      {
      accounts : {
      
        user: user.publicKey,
        userATokenAta: userUSDC,
        userBTokenAta: userUSDT,
        // userATokenAta: 
        // Vault Stuff
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
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        lendingProgram: new anchor.web3.PublicKey("4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM"),
        //lendingProgram:  new anchor.web3.PublicKey("LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi"),
        sysVarClock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        // Tulip Accounts
        destinationCollateral: new anchor.web3.PublicKey("2U6kk4iTVqeypBydVPKA8mLTLAQEBfWf4KYfmkcvomPE"),
        reserveAccount: new anchor.web3.PublicKey("FTkSmGsJ3ZqDSHdcnY7ejN1pWV3Ej7i88MYpZyyaqgGt"),
        reserveLiquiditySupply: new anchor.web3.PublicKey("64QJd6MYXUjCBvCaZKaqxiKmaMkPUdNonE1KuY1YoGGb"),
        lendingMarket: new anchor.web3.PublicKey("D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym"),
        lendingMarketAuthority: new anchor.web3.PublicKey("8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s"),
      },
      signers : [user]
    })
    console.log("tx", tx)

  })

  it("User Withdraw!", async () => {

    const user = vaultAdmin;

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

    const tx = await program.rpc.withdraw(
      
      // Instruction Arguments
      [infoBump, authorityBump, tokenABump, tokenBBump, tutokenABump, tutokenBBump],
      1, 
      1,
      // Accounts
      {
      accounts : {
        
        user: user.publicKey,
        // Vault Stuff
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
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      },
      signers : [user]
    })

  })

  it("User Swap!", async () => {

    const user = vaultAdmin;

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

    const tx = await program.rpc.swap(
      
      // Instruction Arguments
      [infoBump, authorityBump, tokenABump, tokenBBump, tutokenABump, tutokenBBump],
      1, 
      1,
      false,
      // Accounts
      {
      accounts : {
        
        user: user.publicKey,
        // Vault Stuff
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
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      },
      signers : [user]
    })

  })
});
