const assert = require("assert");
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Splendor } from "../target/types/splendor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

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
  bumps = [255,255,255,255,255,255];
  it("Vault initialized!", async () => {

    // Add your test here.
    const tx = await program.rpc.initializeVault("my_vault", bumps, {
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
      },
      signers : [vaultAdmin]
    });
    console.log("Your transaction signature", tx);


    //let vaultAccount = await program.account.vault_info.fetch(vault_info.publicKey);
    let vaultAccount = await program.account.vaultBumps.fetch(vaultBumps.publicKey)
  })
});
