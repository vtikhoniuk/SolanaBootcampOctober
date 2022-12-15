import React, {useState, useEffect, useCallback} from 'react'
import cl from './Game.module.css'
import {useConnection, useWallet} from '@solana/wallet-adapter-react'
import {Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction, TransactionSignature, 
    sendAndConfirmTransaction, LAMPORTS_PER_SOL} from '@solana/web3.js'
import { notify } from "../utils/notifications"
import * as BufferLayout from 'buffer-layout';
import { Buffer } from 'buffer';

function Square(props) {
    return (
        <button className={cl.square} onClick={props.onClick}>
            {props.value}
        </button>
    );
}

function Board(props) {
    function renderSquare(i) {
        return (
            <Square
                value={props.square[i]}
                onClick={() => props.onClick(i)}
            />
        )
    }

    return (
        <div>
            <div className={cl.boardRow}>
                {renderSquare(0)}
                {renderSquare(1)}
                {renderSquare(2)}
            </div>
            <div className={cl.boardRow}>
                {renderSquare(3)}
                {renderSquare(4)}
                {renderSquare(5)}
            </div>
            <div className={cl.boardRow}>
                {renderSquare(6)}
                {renderSquare(7)}
                {renderSquare(8)}
            </div>
        </div>
    );
}

const COMMAND_LENGTH = 8

function Game() {
    const [square, setSquare] = useState(Array(9).fill(null))
    const [xIsNext, setXIsNext] = useState(true)

    const {publicKey, sendTransaction} = useWallet();
    const {connection} = useConnection();

    const programPubkey = new PublicKey("DduGXe34xu1JsSJLot9cVTrQDdNqVfZaFq4W6F7s88uN")

    const playerTwo = Keypair.generate()
    const game = Keypair.generate()

    function createGameTransactionData(): Buffer {
        const dataLayout = BufferLayout.struct([
          BufferLayout.u32('instruction')
        ])
      
        const data = Buffer.alloc(dataLayout.span)
        dataLayout.encode({instruction: 0}, data)

        return data
    }

    const createGame = useCallback( async () => {
        if (!publicKey) {
            return;
        }

        console.log("game.PublicKey: " + game.publicKey)

        let airdropSignature = await connection.requestAirdrop(
            game.publicKey,
            LAMPORTS_PER_SOL,
        );
        
        await connection.confirmTransaction(airdropSignature)

        let createGameTransaction = new Transaction().add(
            new TransactionInstruction({
                keys: [
                    {pubkey: game.publicKey, isSigner: true, isWritable: true},
                    {pubkey: publicKey, isSigner: false, isWritable: true},
                    {pubkey: programPubkey, isSigner: false, isWritable: true}
                ],
                programId: programPubkey,
                data: createGameTransactionData()
            })
        )

        await sendAndConfirmTransaction(connection, createGameTransaction, [game]);
    },[publicKey])

    useEffect(() => {
        createGame()
    },[publicKey])

    function handleClick(i) {
        if (square[i]) {
            return
        }
        square.splice(i, 1, xIsNext ? "X" : "O")
        setSquare(square)
        setXIsNext(!xIsNext);
    }

    /* render() {        
        const winner = false;

        let status;
        if (winner) {
            status = "Winner: " + winner;
        } else {
            status = "Next player: " + (this.state.xIsNext ? "X" : "O");
        } */

    return (
        <div className={cl.game}>
            <div className={cl.gameBoard}>
                <Board
                    square={square.slice()}
                    onClick={i => handleClick(i)}
                />
            </div>
            <div className={cl.gameInfo}>
                {"Next player: " + (xIsNext ? "X" : "O")}
            </div>
        </div>
    );
    
}

export default Game