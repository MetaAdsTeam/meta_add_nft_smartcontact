// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::{AccountId, Promise, Balance, env, near_bindgen, log, setup_alloc, Timestamp};
use near_sdk::collections::{UnorderedMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MIN_DEPOSIT: Balance = 1_000_000_000_000_000_000_000;

setup_alloc!();

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Unit {
    pub record_id: u64,
    pub name: String,
    pub content: String,
    pub owner_account_id: AccountId,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Slot {
    pub record_id: u64,
    pub space_id: u64,
    pub unit_id: u64,
    pub amount: Balance,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub transfered: bool,
    pub advertiser_account_id: AccountId,
    pub publisher_account_id: AccountId,
}

type Units = UnorderedMap<u64, Unit>;
type Slots = UnorderedMap<u64, Slot>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MetaAdsContract {
    units: Units,
    slots: Slots,
    max_unit_id: u64,
    max_slot_id: u64,
}

impl Default for MetaAdsContract {
    fn default() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            units: UnorderedMap::new(b"u".to_vec()),
            slots: UnorderedMap::new(b"s".to_vec()),
            max_unit_id: 0,
            max_slot_id: 0,
        }
    }
}


#[near_bindgen]
impl MetaAdsContract {
    pub fn make_unit(&mut self, name: String, content: String) -> Unit {
    
        assert!(name != "", "Abort. Name is empty");
        assert!(name.len() <= 100, "Abort. Name is longer then 100 characters");
        assert!(content != "", "Abort. Content is empty");

        let owner_account_id: AccountId = env::predecessor_account_id();
        self.max_unit_id += 1;
        let record_id = self.max_unit_id;
        let unit = Unit {
            record_id,
            name,
            content,
            owner_account_id,
        };

        self.units.insert(
            &record_id,
            &unit,
        );

        unit
    }

    #[payable]
    pub fn take_slot(&mut self, space_id: u64, unit_id: u64, start_time: Timestamp, end_time: Timestamp, publisher_id: AccountId) -> Option<Slot> {
        
        assert!(space_id > 0, "Abort. Space Id undefined");
        assert!(unit_id > 0, "Abort. Unit Id undefined");

        let time: u64 = env::block_timestamp() / 1000000000;
        assert!(start_time >= time, "Abort. Start time is less than current");
        assert!(end_time > time, "Abort. End time is less than current");
        assert!(end_time > start_time, "Abort. Start time must be less than End time");
        assert!(publisher_id != "", "Abort. Publisher is empty");

        let deposit: Balance = env::attached_deposit();
        assert!(deposit >= MIN_DEPOSIT, "Deposit is too small. Attached: {}, Required: {}", deposit, MIN_DEPOSIT);

        match self.units.get(&unit_id) {
            Some(_unit) => {
                
                let advertiser_account_id = env::predecessor_account_id();
                let owner_account_id = _unit.owner_account_id.clone();
                assert_eq!(owner_account_id, advertiser_account_id, "Abort. Unit not available. Wrong account");
                
                self.max_slot_id += 1;
                let record_id = self.max_slot_id;
                let slot = Slot {
                    record_id,
                    space_id,
                    unit_id,
                    amount: deposit.into(),
                    start_time,
                    end_time,
                    transfered: false,
                    advertiser_account_id: advertiser_account_id.clone(),
                    publisher_account_id: publisher_id,
                };
                
                self.slots.insert(
                    &record_id,
                    &slot,
                );
                
                Some(slot)
            }
            None => None
        }
    }

    pub fn transfer_funds(&mut self, slot_id: u64) -> bool {
        
        assert!(slot_id > 0, "Abort. Slot Id undefined");
        
        match self.slots.get(&slot_id) {
            Some(mut slot) => {
                
                assert!(slot.transfered == false, "Abort. Transfer funds");

                let time: u64 = env::block_timestamp() / 1000000000;
                assert!(slot.end_time <= time, "Abort. Slot is active. Show time is not over yet");

                let amount = slot.amount;
                let fee = amount / 10;
        
                let total_funds: Balance = amount - fee;
                let account_id = slot.publisher_account_id.clone();
        
                Promise::new(account_id.clone()).transfer(total_funds);
                
                log!("Publisher is {}. transferred funds: {}", account_id.clone(), total_funds);

                slot.transfered = true;
                self.slots.insert(&slot_id, &slot);
                
                true
            }
            None => {
                false
            }
        }
    }

    pub fn fetch_all_units(&self) -> HashMap<u64, Unit> {
        self.units.iter().collect()
    }

    pub fn fetch_unit_by_id(&self, id: u64) -> Unit {
        self.units.get(&id).expect("Unit not found")
    }

    pub fn fetch_all_slots(&self) -> HashMap<u64, Slot> {
        self.slots.iter().collect()
    }

    pub fn fetch_slot_by_id(&self, id: u64) -> Slot {
        self.slots.get(&id).expect("Slot not found")
    }
}


