import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Splendorprotocol } from "../target/types/splendorprotocol";

describe("splendorprotocol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Splendorprotocol as Program<Splendorprotocol>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});