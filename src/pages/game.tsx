import type { NextPage } from "next";
import Head from "next/head";
import React from "react";
import { GameView } from "../views";

const Game: NextPage = (props) => {
  return (
    <div>
      <Head>
        <title>Solana Scaffold</title>
        <meta
          name="description"
          content="Game Functionality"
        />
      </Head>
      <GameView />
    </div>
  );
};

export default Game;
