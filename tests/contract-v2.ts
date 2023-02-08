import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ContractV2 } from "../target/types/contract_v2";

describe("contract-v2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ContractV2 as Program<ContractV2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
