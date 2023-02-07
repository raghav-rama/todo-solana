import * as anchor from "@project-serum/anchor";
import { TodoSolana } from "../target/types/todo_solana";

describe("todo-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TodoSolana as anchor.Program<TodoSolana>;

  let account: anchor.web3.PublicKey = anchor.AnchorProvider.env().wallet.publicKey;

  // todo item account
  let keypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();

  //genesis todo item account
  let genesisKeyPair: anchor.web3.Keypair = anchor.web3.Keypair.generate();

  //next todo item account
  let nextKeyPair: anchor.web3.Keypair = anchor.web3.Keypair.generate();

  let secretKey = anchor.Wallet.local().payer.secretKey;

  let myKeyPair = anchor.web3.Keypair.fromSecretKey(secretKey);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        todoItem: keypair.publicKey,
        nextTodoAccount: nextKeyPair.publicKey,
        genesisTodoAccount: genesisKeyPair.publicKey,
        user: account,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([keypair, genesisKeyPair, nextKeyPair, myKeyPair])
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("Anchor provider", account.toBase58());
    console.log("Keypair", keypair.publicKey.toBase58());

    const _account = await program.provider.connection.getParsedAccountInfo(keypair.publicKey);

    console.log("Account", _account);

    const todoItem = await program.account.todoItem.fetch(keypair.publicKey);

    console.log("Todo Item", todoItem);

    const head = await program.account.head.fetch(genesisKeyPair.publicKey);

    console.log("Head", head);

    const currentTodoItem = await program.account.currentTodoItem.fetch(nextKeyPair.publicKey);

    console.log("Current Todo Item", currentTodoItem);
  });
});
