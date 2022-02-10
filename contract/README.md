Meta Ads Smart Contract
==================

A [smart contract] written in [Rust] for an app initialized with [create-near-app]


Quick Start
===========

Before you compile this code, you will need to install Rust with [correct target]


Exploring The Code
==================

 - The main smart contract code lives in `src/lib.rs`. You can compile it with
   the `./compile` or `./build.sh` script.


How to deploy
==================

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as ` YOUR-NAME.testnet`, you can deploy your contract to `subaccount.YOUR-NAME.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `subaccount.YOUR-NAME.testnet`:

- Authorize NEAR CLI, following the commands it gives you:

      `near login`

- Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      `near create-account subaccount.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet`

- Set contract to subaccount

      `near deploy --accountId subaccount.YOUR-NAME.testnet --wasmFile=./out/main.wasm `


Examples
==================

- Add creative

   `near call subaccount.YOUR-NAME.testnet make_creative '{"name": "My Creative", "content": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gi", "nft_cid": $cid}' --accountId YOUR-NAME.testnet`

- Get a collection of creatives

   `near view subaccount.YOUR-NAME.testnet fetch_all_creatives`

- Get a creative by id

   `near view subaccount.YOUR-NAME.testnet fetch_creative_by_id '{"id": $id}'`

- Init presentation your creative

   `near call subaccount.YOUR-NAME.testnet do_agreement '{"adspace_id": $sid, "creative_id": $сid, "start_time": $s_time, "end_time": $e_time, "publisher_id": "'$PublisherAccountId'"}' --accountId  YOUR-NAME.testnet --amount 0.1`

- Transfer of funds to the publisher for presentation

   `near call subaccount.YOUR-NAME.testnet transfer_funds '{"presentation_id": $pid}' --accountId  subaccount.YOUR-NAME.testnet`


  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
