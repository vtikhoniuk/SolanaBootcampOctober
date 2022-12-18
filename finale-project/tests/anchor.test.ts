// No imports needed: web3, anchor, pg and more are globally available


describe("Test", () => {
  it("newGame", async () => {
    const playerone = pg.wallet;
    // Generate keypair for the new account
    const playertwo = web3.Keypair.generate();
    const game = web3.Keypair.generate();

    // Send transaction
    const txHash = await pg.program.methods
      .newGame(playertwo.publicKey)
      .accounts({
        game: game.publicKey,
        player1: playerone.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([game])
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch the created account
    const created_game_account = await pg.program.account.game.fetch(
      game.publicKey
    );

    console.log("On-chain data is:", created_game_account.players[0].toString());
    console.log("On-chain data is:", created_game_account.players[1].toString());
    console.log("On-chain data is:", created_game_account.playerOneMovePos.toString());
    console.log("On-chain data is:", created_game_account.playerTwoMovePos.toString());
    console.log("On-chain data is:", created_game_account.winner.toString());

    const txHash = await pg.program.methods
      .selectPoint("1", "2")
      .accounts({
        game: game.publicKey,
        player: playertwo.publicKey,
      })
      .signers([playertwo])
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    const txHash = await pg.program.methods
      .selectPoint("1", "1")
      .accounts({
        game: game.publicKey,
        player: playerone.publicKey,
      })
      .signers([playerone])
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch the created account
    const created_game_account = await pg.program.account.game.fetch(
      game.publicKey
    );

    console.log("On-chain data is:", created_game_account.players[0].toString());
    console.log("On-chain data is:", created_game_account.players[1].toString());
    console.log("On-chain data is:", created_game_account.playerOneMovePos.toString());
    console.log("On-chain data is:", created_game_account.playerTwoMovePos.toString());
    console.log("On-chain data is:", created_game_account.winner.toString());

    // Check whether the data on-chain is equal to local 'data'
    // assert(playerone..eq(created_game_account.players[0]));
  });
});
