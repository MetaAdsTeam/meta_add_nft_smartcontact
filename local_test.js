const nearAPI = require("near-api-js");
const BN = require("bn.js");
const fs = require("fs").promises;
const assert = require("assert").strict;


const CONTRACT_NAME = process.env.CONTRACT_NAME ||'metaads.test.near'

function getConfig(env) {
  switch (env) {
  case 'testnet':
    return {
      networkId: 'testnet',
      nodeUrl: 'https://rpc.testnet.near.org',
      contractName: CONTRACT_NAME,
      walletUrl: 'https://wallet.testnet.near.org',
      helperUrl: 'https://helper.testnet.near.org',
      explorerUrl: 'https://explorer.testnet.near.org',
    }
  case "local":
    return {
      networkId: process.env.NEAR_CLI_LOCALNET_NETWORK_ID || 'local',
      nodeUrl: process.env.NEAR_NODE_URL || 'http://localhost:3030',
      keyPath: process.env.NEAR_CLI_LOCALNET_KEY_PATH || `${process.env.HOME}/.near/validator_key.json`,
      walletUrl: process.env.NEAR_WALLET_URL || 'http://localhost:4000/wallet',
      contractAccount: CONTRACT_NAME,
      masterAccount: "test.near",
    }
  case "sandbox":
    return {
      networkId: "sandbox",
      nodeUrl: "http://localhost:3030",
      masterAccount: "test.near",
      contractAccount: CONTRACT_NAME,
      keyPath: "/tmp/near-sandbox/validator_key.json",
    }
  default:
    throw Error(`Unconfigured environment '${env}'. Can be configured in src/config.js.`)
  }
}

const contractMethods = {
    viewMethods: ["fetch_all_units", "fetch_unit_by_id", "fetch_all_slots", "fetch_slot_by_id"],
    changeMethods: ["make_unit", "take_slot", "transfer_funds"],
};

let config;
let masterAccount;
let masterAccountContract;
let masterKey;
let pubKey;
let keyStore;
let near;
let adUseContract;
let pubUseContract;
let contractAccount;

async function initNear() {
  config = getConfig(process.env.NEAR_ENV || "local");
  const keyFile = require(config.keyPath);
  masterKey = nearAPI.utils.KeyPair.fromString(
    keyFile.secret_key || keyFile.private_key
  );
  pubKey = masterKey.getPublicKey();
  keyStore = new nearAPI.keyStores.InMemoryKeyStore();
  keyStore.setKey(config.networkId, config.masterAccount, masterKey);
  near = await nearAPI.connect({
    deps: {
      keyStore,
    },
    networkId: config.networkId,
    nodeUrl: config.nodeUrl,
  });
  masterAccount = new nearAPI.Account(near.connection, config.masterAccount);
  masterAccountContract = new nearAPI.Contract(
    masterAccount,
    config.contractAccount,
    contractMethods
  );
  console.log("Finish init NEAR");
}

async function createContractUser(
  accountPrefix,
  contractAccountId,
  contractMethods
) {
  let accountId = accountPrefix + "." + config.masterAccount;
  let account;
  let response;

  try {
    account = new nearAPI.Account(near.connection, accountId);
    response = await account.state();
    keyStore.setKey(config.networkId, accountId, masterKey);
  } catch (err) {
    await masterAccount.createAccount(
      accountId,
      pubKey,
      new BN(10).pow(new BN(25))
    );

    keyStore.setKey(config.networkId, accountId, masterKey);
    account = new nearAPI.Account(near.connection, accountId);
    response = await account.state();
    console.log("Created account "+accountId);
  } 

  console.log("Gets the state for "+accountId); 
  console.log(response); 

  const accountUseContract = new nearAPI.Contract(
    account,
    contractAccountId,
    contractMethods
  );
  return accountUseContract;
}

async function initTest() {

  try {
    let account = new nearAPI.Account(near.connection, config.contractAccount);
    keyStore.setKey(config.networkId, config.contractAccount, masterKey);
    const response = await account.state();
    console.log("Gets the state for " + config.contractAccount); 
    console.log(response); 

    contractAccount = new nearAPI.Contract(
      account,
      config.contractAccount,
      contractMethods
    );

  } catch (err) {
    const contract = await fs.readFile("./out/main.wasm");
    let account = await masterAccount.createAndDeployContract(
      config.contractAccount,
      pubKey,
      contract,
      new BN(10).pow(new BN(25))
    );
    keyStore.setKey(config.networkId, config.contractAccount, masterKey);

    contractAccount = new nearAPI.Contract(
      account,
      config.contractAccount,
      contractMethods
    );
    console.log("Created account and deploy smart contract");
  }

  adUseContract = await createContractUser(
    "ad",
    config.contractAccount,
    contractMethods
  );

  pubUseContract = await createContractUser(
    "pub",
    config.contractAccount,
    contractMethods
  );
  return { adUseContract, pubUseContract };
}

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function testMakeUnit() {

  let name = "Unit #"+(Math.random()).toString(36).substring(10);
  let content = "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif";

  let unit = await adUseContract.make_unit({ args: { name: name, content: content} });
  
  assert(unit, 'Missing unit');

  let ad_unit = await adUseContract.fetch_unit_by_id({
    id: unit.record_id,
  });

  assert.equal(ad_unit.name, name, "Unit name is not equal to the original");
  assert.equal(ad_unit.content, content, "Content is not equal to the original");
  console.log("Unit created");

  return unit;
}

async function testTakeSlot(unit) {

  if(!unit) return;

  let start_time = Math.floor((Date.now() / 1000) + 10);
  let end_time = start_time + 5;
  let publisher_id = "pub." + config.masterAccount;
  const price_near = new BN("11000000000000000000000");

  let slot = await adUseContract.take_slot( 
      { 
        args: { space_id: 1, unit_id: unit.record_id, start_time: start_time, end_time: end_time, publisher_id: publisher_id},
        gas: 10000000000000,
        amount: price_near.toString()
      }
  );

  assert.equal(slot.unit_id, unit.record_id, "Unit ID is not equal to the original");
  assert.equal(slot.start_time, start_time, "Start time is not equal to the original");
  assert.equal(slot.end_time, end_time, "End time is not equal to the original");
  assert.equal(slot.publisher_account_id, publisher_id, "Publisher Id is not equal to the original");

  console.log("Slot busy");
  console.log(slot);

  return slot;
}

async function testTransferFund(slot) {
  let result = await contractAccount.transfer_funds({ args: { slot_id: slot.record_id}});
  assert(result, 'Transfer not completed');

  console.log("Transfer completed");

  await adUseContract.transfer_funds({ args: { slot_id: slot.record_id}});

}

async function test() {
  // 1. Creates testing accounts and deploys a contract
  await initNear();
  await initTest();
  console.log("Finish create test accounts");

  // 2. Performs a `make_unit` transaction signed by Advertiser and then calls `fetch_unit_by_id` to confirm `make_unit` worked
  let unit = await testMakeUnit();
  if(!unit) return;
 
  // 3. Calls `fetch_all_units` to fetch Units collection 
  let units = await masterAccountContract.fetch_all_units();
  assert(units, 'Missing unit collection');
  if(!units) return;

  assert(units[unit.record_id], 'Missing unit in collection');

  // 4. Calls `take_slot` to fetch Slots collection 
  let slot = await testTakeSlot(unit);

  let slots = await masterAccountContract.fetch_all_slots();
  if(!slots) return;
  assert(slots[slot.record_id], 'Missing slot in collection');

   // 5. Calls `transfer_funds`
   await sleep(17000);

   await testTransferFund(slot);

}

test();
