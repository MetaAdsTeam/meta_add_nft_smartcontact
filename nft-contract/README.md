Meta Ads Smart Contract
==================

A [nft-contract] written in [Rust] for an app initialized with [nft-tutorial]


Quick Start
===========

Before you compile this code, you will need to install Rust with [correct target]


Exploring The Code
==================

 - The main smart contract code lives in `src/lib.rs`. You can compile it with
   the `./compile` or `./build.sh` script.

How to deploy
==================

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `YOUR-NAME.testnet`, you can deploy your contract to `nft.YOUR-NAME.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `nft.YOUR-NAME.testnet`:

- Authorize NEAR CLI, following the commands it gives you:

      `near login`

- Create a subaccount for NFT contract (replace `YOUR-NAME` below with your actual account name):

      `near create-account nft.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet`

- Set NFT contract to subaccount

      `near deploy --accountId nft.YOUR-NAME.testnet  --wasmFile=./out/nft_contract.wasm`

- Initialize NFT contract

      `near call nft.YOUR-NAME.testnet new_default_meta '{"owner_id": "nft.YOUR-NAME.testnet"}' --accountId nft.YOUR-NAME.testnet`

- View Contracts Meta Data

      `near view nft.YOUR-NAME.testnet nft_metadata`

- Minting Token

      `near call nft.YOUR-NAME.testnet nft_mint '{"token_id": "token-1", "metadata": {"title": "Test Token 1", "description": "Test Token", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "YOUR-NAME.testnet", "webdata": {"uri":"https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}}' --accountId YOUR-NAME.testnet --amount 0.1`

- View NFT Information

      `near view nft.YOUR-NAME.testnet nft_token '{"token_id": "token-1"}'`

- Get the total supply of NFTs for a given owner

      `near view nft.YOUR-NAME.testnet nft_supply_for_owner '{"account_id": "OWNER-NAME.testnet"}'`

- Get all tokens for an owner 

      `near view nft.YOUR-NAME.testnet nft_tokens_for_owner '{"account_id": "OWNER-NAME.testnet", "limit": $limit}'`   


[nft-contract]: https://docs.near.org/es-ES/docs/tutorials/contracts/nfts/introduction
[Rust]: https://www.rust-lang.org/
[nft-tutorial]: https://github.com/near-examples/nft-tutorial
[correct target]: https://github.com/near/near-sdk-rs#pre-requisites