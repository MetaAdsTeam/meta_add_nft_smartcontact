// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::{AccountId, Promise, Balance, env, near_bindgen, log, setup_alloc, Timestamp};
use near_sdk::collections::{UnorderedMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 1 NEAR
const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

setup_alloc!();

pub use crate::creative::*;
pub use crate::presentation::*;
pub use crate::ad_spot::*;

mod ad_spot;
mod creative;
mod presentation;

#[derive(BorshSerialize)]
pub enum StorageKey {
    Creatives,
    Presentations,
    AdSpot,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MetaAdsContract {
    pub creatives: UnorderedMap<u64, Creative>,
    pub presentations: UnorderedMap<u64, Presentation>,
    pub ad_spots: UnorderedMap<u64, AdSpot>,
}

impl Default for MetaAdsContract {
    fn default() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            creatives: UnorderedMap::new(StorageKey::Creatives.try_to_vec().unwrap()),
            presentations: UnorderedMap::new(StorageKey::Presentations.try_to_vec().unwrap()),
            ad_spots: UnorderedMap::new(StorageKey::AdSpot.try_to_vec().unwrap()),
        }
    }
}


#[near_bindgen]
impl MetaAdsContract {

    #[private]
    pub fn transfer_funds(&mut self, playback_id: u64) -> bool {
        
        assert!(playback_id > 0, "Abort. Presentation Id undefined");

        match self.presentations.get(&playback_id) {
            Some(mut presentation) => {
                
                assert!(presentation.transfered == false, "Abort. Transfer funds");

                let time: u64 = env::block_timestamp() / 1000000000;
                assert!(presentation.end_time <= time, "Abort. Presentation is active. Show time is not over yet");
        
                let total_funds: Balance = presentation.advertiser_cost - presentation.entertainment_fee;
                let account_id = presentation.publisher_account_id.clone();
        
                Promise::new(account_id.clone()).transfer(total_funds);
                
                log!("The publisher {} received funds in the amount of {}", account_id.clone(), total_funds);

                presentation.transfered = true;
                presentation.status = String::from("success");
                self.presentations.insert(&playback_id, &presentation);
                
                true
            }
            None => {
                false
            }
        }
    }
}


