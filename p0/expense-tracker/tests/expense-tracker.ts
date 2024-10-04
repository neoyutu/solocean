import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { ExpenseTracker } from "../target/types/expense_tracker";

describe("expense-tracker", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.local();

  const program = anchor.workspace.ExpenseTracker as Program<ExpenseTracker>;
  const wallet = provider.wallet as anchor.Wallet;

  let merchantName = "test";
  let amount = 100;
  let id = 1;

  let merchantName2 = "test 2";
  let amount2 = 200;

  let [expense_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("expense"),
      wallet.publicKey.toBuffer(),
      new BN(id).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeExpense(
      new anchor.BN(id),
      merchantName,
      new anchor.BN(amount)
    ).accounts({
      expenseAccount: expense_account,
      signer: wallet.publicKey,
    })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Check expenses!", async () => {
    const expenses = await program.account.expenseAccount.all([
      {
        memcmp: {
          offset: 8 + 8,
          bytes: wallet.publicKey.toBase58(),
        },
      },
    ]);
    console.log(expenses.map((v) => v.account))
  });

  it("Modify Expense", async () => {
    await program.methods
      .modifyExpense(new anchor.BN(id), merchantName2, new anchor.BN(amount2))
      .accounts({
        expenseAccount: expense_account,
        authority: wallet.publicKey,
      })
      .rpc();
  });

  it("Delete Expense", async () => {
    await program.methods
      .deleteExpense(new anchor.BN(id))
      .accounts({
        expenseAccount: expense_account,
        authority: wallet.publicKey,
      })
      .rpc();
  });
});
