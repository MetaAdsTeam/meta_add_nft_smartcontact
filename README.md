Meta Ads
==================

This app was initialized with [create-near-app]


Quick Start
===========

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `yarn install`
3. Run the local development server: `yarn dev` (see `package.json` for a
   full list of `scripts` you can run with `yarn`)

Now you'll have a local development environment backed by the NEAR TestNet!

Go ahead and play with the app and the code. As you make code changes, the app will automatically reload.


Exploring The Code
==================

The "backend" code lives in the `/contract` folder. See the README there for more info.


Deploy
======

Step 0: Install near-cli (optional)
-------------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Creating Accounts for Contracts and Deploying Contracts
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `subaccount.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `subaccount.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      `near login`

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      `near create-account subaccount.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet`

3. Set contract to subaccount

      `near deploy --accountId subaccount.YOUR-NAME.testnet --wasmFile=./out/main.wasm `

4. Create a subaccount for NFT contract (replace `YOUR-NAME` below with your actual account name):

      `near create-account nft.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet`

5. Set NFT contract to subaccount

      `near deploy --accountId nft.YOUR-NAME.testnet  --wasmFile=./out/nft_contract.wasm`

6. Initialize NFT contract

      `near call nft.YOUR-NAME.testnet new_default_meta '{"owner_id": "nft.YOUR-NAME.testnet"}' --accountId nft.YOUR-NAME.testnet`

7. View Contracts Meta Data

      `near view nft.YOUR-NAME.testnet nft_metadata`

8. Minting Token

      `near call nft.YOUR-NAME.testnet nft_mint '{"token_id": "token-1", "metadata": {"title": "Test Token 1", "description": "Test Token", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "YOUR-NAME.testnet", "webdata": {"uri":"https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}}' --accountId YOUR-NAME.testnet --amount 0.1`

9. View NFT Information

      `near view nft.YOUR-NAME.testnet nft_token '{"token_id": "token-1"}'`

10. Get the total supply of NFTs for a given owner

      `near view nft.YOUR-NAME.testnet nft_supply_for_owner '{"account_id": "OWNER-NAME.testnet"}'`

11. Get all tokens for an owner 

      `near view nft.YOUR-NAME.testnet nft_tokens_for_owner '{"account_id": "OWNER-NAME.testnet", "limit": $limit}'`


Troubleshooting
===============

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


  [create-near-app]: https://github.com/near/create-near-app
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [jest]: https://jestjs.io/
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [gh-pages]: https://github.com/tschaub/gh-pages
