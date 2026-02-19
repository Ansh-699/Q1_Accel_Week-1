import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Week1TransferHookVault } from "../target/types/week1_transfer_hook_vault";
import { expect } from "chai";

const TOKEN_2022_PROGRAM_ID = new anchor.web3.PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

describe("week1-transfer-hook-vault", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const admin = provider.wallet.publicKey;

  const program = anchor.workspace.week1TransferHookVault as Program<Week1TransferHookVault>;
  const [vaultConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_config")],
    program.programId,
  );

  const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultConfigPda.toBuffer()],
    program.programId,
  );

  it("Is initialized!", async () => {
    const vaultConfigInfo = await provider.connection.getAccountInfo(vaultConfigPda);

    if (!vaultConfigInfo) {
      const mint = anchor.web3.Keypair.generate();

      await program.methods.initialize().accountsPartial({
        admin,
        vaultConfig: vaultConfigPda,
        mint: mint.publicKey,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      }).signers([mint]).rpc();
    }

    const vaultConfig = await program.account.vaultLedger.fetch(vaultConfigPda);
    expect(vaultConfig.admin.toBase58()).to.eq(admin.toBase58());
    expect(vaultConfig.whitelist.length).to.eq(0);
  });
});
