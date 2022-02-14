// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::{AccountId, Promise, Balance, env, near_bindgen, log, setup_alloc, Timestamp};
use near_sdk::collections::{UnorderedMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 1 NEAR
const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Creative {
    pub creative_id: u64,
    pub name: String,
    pub content: String,
    pub nft_cid: Option<String>,
    pub owner_account_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Presentation {
    pub playback_id: u64,
    pub adspot_id: u64,
    pub creative_id: u64,
    pub advertiser_cost : Balance,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub transfered: bool,
    pub advertiser_account_id: AccountId,
    pub publisher_account_id: AccountId,
    pub ad_spot_name: String,
    pub publisher_earn: Option<u64>,
    pub show_kind: Option<String>,
    pub entertainment: String,
    pub entertainment_fee: Balance,
    pub status: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AdSpot {
    pub adspot_id: u64,
    pub owner_account_id: AccountId,
    pub price: Balance,
    pub name: String,
    pub publisher_earn: Option<u64>,
    pub show_kind: Option<String>, 
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
            creatives: UnorderedMap::new(b"c".to_vec()),
            presentations: UnorderedMap::new(b"p".to_vec()),
            ad_spots: UnorderedMap::new(b"s".to_vec()),
        }
    }
}


#[near_bindgen]
impl MetaAdsContract {
    pub fn make_creative(&mut self, name: String, content: String, creative_id: u64, nft_cid: Option<String>) -> Creative {
    
        assert!(name != "", "Abort. Name is empty");
        assert!(name.len() <= 100, "Abort. Name is longer than 100 characters");
        assert!(content != "", "Abort. Content is empty");
        assert!(creative_id > 0, "Abort. Creative Id undefined");

        let owner_account_id: AccountId = env::predecessor_account_id();
        let creative = Creative {
            creative_id,
            name,
            content,
            nft_cid,
            owner_account_id,
        };

        assert!(
            self.creatives.insert(&creative_id, &creative).is_none(),
            "Creative already exists"
        );

        creative
    }

    pub fn make_adspot(
        &mut self, 
        adspot_id: u64, 
        price: Balance,
        name: String, 
        publisher_earn: Option<u64>, 
        show_kind: Option<String>
    ) -> AdSpot {
        
        assert!(adspot_id > 0, "Abort. Playback Id undefined");
        assert!(price > 0, "Abort. Price undefined");
        assert!(name != "", "Abort. Name is empty");
        assert!(name.len() <= 100, "Abort. Name is longer than 100 characters");

        let owner_account_id: AccountId = env::predecessor_account_id();
        let ad_spot = AdSpot {
            adspot_id,
            owner_account_id,
            price: price * ONE_NEAR,
            name,
            publisher_earn,
            show_kind,
        };

        assert!(
            self.ad_spots.insert(&adspot_id, &ad_spot).is_none(),
            "Ad Spot already exists"
        );

        ad_spot
    }

    #[payable]
    pub fn do_agreement(
        &mut self, 
        playback_id: u64,
        adspot_id: u64, 
        creative_id: u64,  
        start_time: Timestamp, 
        end_time: Timestamp,
    ) -> Option<Presentation> {
        
        assert!(playback_id > 0, "Abort. Playback Id undefined");
        assert!(adspot_id > 0, "Abort. AdSpace Id undefined");
        assert!(creative_id > 0, "Abort. Creative Id undefined");

        let time: u64 = env::block_timestamp() / 1000000000;
        assert!(start_time >= time, "Abort. Start time is less than current time");
        assert!(end_time > time, "Abort. End time is less than current time");
        assert!(end_time > start_time, "Abort. Start time must be less than End time");

        match self.creatives.get(&creative_id) {
            Some(_creative) => {

                match self.ad_spots.get(&adspot_id) {

                    Some(_adspot) => {
                        
                        let deposit: Balance = env::attached_deposit();
                        assert!(deposit >= _adspot.price, "Deposit is too small. Attached: {}, Required: {}", deposit, _adspot.price);

                        let advertiser_account_id = env::predecessor_account_id();
                        let owner_account_id = _creative.owner_account_id.clone();
                        assert_eq!(owner_account_id, advertiser_account_id, "Abort. Creative not available. Wrong account");

                        let fee = deposit / 10;

                        let presentation = Presentation {
                            playback_id,
                            adspot_id,
                            creative_id,
                            advertiser_cost: deposit.into(),
                            start_time,
                            end_time,
                            transfered: false,
                            advertiser_account_id: advertiser_account_id.clone(),
                            publisher_account_id: _adspot.owner_account_id.clone(),
                            ad_spot_name: _adspot.name.clone(),
                            publisher_earn: _adspot.publisher_earn.clone(),
                            show_kind: _adspot.show_kind.clone(),
                            entertainment: env::current_account_id(),
                            entertainment_fee: fee,
                            status: String::from("signed")
                        };

                        assert!(
                            self.presentations.insert(&playback_id, &presentation).is_none(),
                            "Presentation already exists"
                        );
                        
                        Some(presentation)
                    }
                    None => {
                        near_sdk::env::panic(b"Ad Spot not found");
                    } 
                }
            }
            None => {
                near_sdk::env::panic(b"Creative not found");
            }
        }
    }

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

    pub fn fetch_all_creatives(&self) -> HashMap<u64, Creative> {
        self.creatives.iter().collect()
    }

    pub fn fetch_creative_by_id(&self, id: u64) -> Creative {
        self.creatives.get(&id).expect("Creative not found")
    }

    pub fn fetch_all_presentations(&self) -> HashMap<u64, Presentation> {
        self.presentations.iter().collect()
    }

    pub fn fetch_presentation_by_id(&self, id: u64) -> Presentation {
        self.presentations.get(&id).expect("Presentation not found")
    }

    pub fn fetch_all_adspots(&self) -> HashMap<u64, AdSpot> {
        self.ad_spots.iter().collect()
    }

    pub fn fetch_adspot_by_id(&self, id: u64) -> AdSpot {
        self.ad_spots.get(&id).expect("AdSpot not found")
    }
}


