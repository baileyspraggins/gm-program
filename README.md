# gm-program

## Description

- Solana program created during the Chainlink Solana Bootcamp to get comfortable interacting with the Solana Blockchain

## Features

- A Simple CLI client that displays the account, program and client account as well as a message saying hello and greeting the user with Good Morning.

## Concepts Used

- Solana Programs and Accounts
- Serialization and Deserialization
- Solana Web3.JS RPC

## How to interact with this code.

- By default this code is run on a local cluster. If you wish you change to the Solana devnet make sure your cli cluster is set as well as line 28 in utils.js is set to "https://api.devnet.solana.com"
- To se the name of who you would like to say good morning to please enter a string for the name parameter in line 63 in the gm_program.ts file.

## Running the Program

- In the root of the project director run "cargo build-bpf" to build the project.
- Follow the prompt to deploy to your local cluster
- After your program is deployed enter the command npm run start and the program will execute

## Future Additions to project

- Store numbers in the account as well as how many times the account has said GM to it.
