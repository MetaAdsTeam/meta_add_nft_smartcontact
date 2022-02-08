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

- Create Ad Unit

   `near call subaccount.YOUR-NAME.testnet make_unit '{"name": "Ad Unit", "content": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gi"}' --accountId YOUR-NAME.testnet`

- Get a collection of units

   `near view subaccount.YOUR-NAME.testnet fetch_all_units`

- Get a unit by id

   `near view subaccount.YOUR-NAME.testnet fetch_unit_by_id '{"id": $id}'`

- Create Slot

   `near call subaccount.YOUR-NAME.testnet take_slot '{"space_id": $sid, "unit_id": $uid, "start_time": $s_time, "end_time": $e_time, "publisher_id": "'$PublisherAccountId'"}' --accountId  YOUR-NAME.testnet --amount 0.1`

- Transfer Funds

   `near call subaccount.YOUR-NAME.testnet transfer_funds '{"slot_id": $sid}' --accountId  YOUR-NAME.testnet`


  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
