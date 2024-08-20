import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaDuplicateMutable } from "../target/types/solana_duplicate_mutable";
import { assert } from "chai";

describe("solana-duplicate-mutable", () => {
  //setting the provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaDuplicateMutable as Program<SolanaDuplicateMutable>;

  //creating the players
  const playerOne = anchor.web3.Keypair.generate();
  const playerTwo = anchor.web3.Keypair.generate();

  it("Initialize player one.", async () => {
    await program.methods
    .initialize()
    .accounts({
      newPlayer : playerOne.publicKey,
      payer : provider.wallet.publicKey
    })
    .signers([playerOne])
    .rpc();

    console.log("✅Transaction was successful.");
  });

  it("Initialized player two.", async () => {
    await program.methods
      .initialize()
      .accounts({
        newPlayer: playerTwo.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([playerTwo])
      .rpc();
  })

  it("Invoke insecure instruction.", async () => {
    //initialize the game by calling the instruction
    await program.methods
    .rockPaperScissorsShootInsecure(
      {rock : {}}, {scissors : {}}
    )
    .accounts({
      playerOne : playerOne.publicKey,
      playerTwo : playerTwo.publicKey
    })
    .rpc();

    //fetch player one state
    const p1 = await program.account.playerState.fetch(playerOne.publicKey);
    assert.equal(JSON.stringify(p1.choice), JSON.stringify({
      rock : {}
    }));
    assert.notEqual(JSON.stringify(p1.choice), JSON.stringify({
      scissors : {}
    }))
  })

  it("Invoke secure instruction.", async () => {
    //calling the secure rock papers scissors function
    await program.methods
    .rockPaperScissorsShootSecure({ rock: {} }, { scissors: {} })
    .accounts({
        playerOne: playerOne.publicKey,
        playerTwo: playerTwo.publicKey,
    })
    .rpc()

    //checking the program states after the game
    const p1 = await program.account.playerState.fetch(playerOne.publicKey)
    const p2 = await program.account.playerState.fetch(playerTwo.publicKey)
    assert.equal(JSON.stringify(p1.choice), JSON.stringify({ rock: {} }))
    assert.equal(JSON.stringify(p2.choice), JSON.stringify({ scissors: {} }))
})


});
