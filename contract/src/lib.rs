// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::{AccountId, Promise, Balance, env, near_bindgen, log, setup_alloc, Timestamp};
use near_sdk::collections::{UnorderedMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MIN_DEPOSIT: Balance = 1_000_000_000_000_000_000_000;

setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Creative {
    pub record_id: u64,
    pub name: String,
    pub content: String,
    pub nft_cid: Option<String>,
    pub owner_account_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Presentation {
    pub record_id: u64,
    pub adspace_id: u64,
    pub creative_id: u64,
    pub advertiser_cost : Balance,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub transfered: bool,
    pub advertiser_account_id: AccountId,
    pub publisher_account_id: AccountId,
    pub ad_space_name: Option<String>,
    pub publisher_earn: Option<u64>,
    pub creative_ref: Option<String>,
    pub show_kind: Option<String>,
    pub entertainment: String,
    pub entertainment_fee: Balance,
    pub status: String,
}

type Creatives = UnorderedMap<u64, Creative>;
type Presentations = UnorderedMap<u64, Presentation>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MetaAdsContract {
    pub creatives: Creatives,
    pub presentations: Presentations,
    pub max_creative_id: u64,
    pub max_presentation_id: u64,
}

impl Default for MetaAdsContract {
    fn default() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            creatives: UnorderedMap::new(b"c".to_vec()),
            presentations: UnorderedMap::new(b"p".to_vec()),
            max_creative_id: 0,
            max_presentation_id: 0,
        }
    }
}


#[near_bindgen]
impl MetaAdsContract {
    pub fn make_creative(&mut self, name: String, content: String, nft_cid: Option<String>) -> Creative {
    
        assert!(name != "", "Abort. Name is empty");
        assert!(name.len() <= 100, "Abort. Name is longer than 100 characters");
        assert!(content != "", "Abort. Content is empty");

        let owner_account_id: AccountId = env::predecessor_account_id();
        self.max_creative_id += 1;
        let record_id = self.max_creative_id;
        let creative = Creative {
            record_id,
            name,
            content,
            nft_cid,
            owner_account_id,
        };

        self.creatives.insert(
            &record_id,
            &creative,
        );

        creative
    }

    #[payable]
    pub fn do_agreement(
        &mut self, 
        adspace_id: u64, 
        creative_id: u64,  
        start_time: Timestamp, 
        end_time: Timestamp, 
        publisher_id: AccountId, 
        ad_space_name: Option<String>,
        creative_ref: Option<String>, 
        publisher_earn: Option<u64>, 
        show_kind: Option<String>, 
    ) -> Option<Presentation> {
        
        assert!(adspace_id > 0, "Abort. Space Id undefined");
        assert!(creative_id > 0, "Abort. Creative Id undefined");

        let time: u64 = env::block_timestamp() / 1000000000;
        assert!(start_time >= time, "Abort. Start time is less than current time");
        assert!(end_time > time, "Abort. End time is less than current time");
        assert!(end_time > start_time, "Abort. Start time must be less than End time");
        assert!(publisher_id != "", "Abort. Publisher is empty");

        let deposit: Balance = env::attached_deposit();
        assert!(deposit >= MIN_DEPOSIT, "Deposit is too small. Attached: {}, Required: {}", deposit, MIN_DEPOSIT);

        match self.creatives.get(&creative_id) {
            Some(_creative) => {
                
                let advertiser_account_id = env::predecessor_account_id();
                let owner_account_id = _creative.owner_account_id.clone();
                assert_eq!(owner_account_id, advertiser_account_id, "Abort. Creative not available. Wrong account");

                let fee = deposit / 10;
                
                self.max_presentation_id += 1;
                let record_id = self.max_presentation_id;
                let presentation = Presentation {
                    record_id,
                    adspace_id,
                    creative_id,
                    advertiser_cost: deposit.into(),
                    start_time,
                    end_time,
                    transfered: false,
                    advertiser_account_id: advertiser_account_id.clone(),
                    publisher_account_id: publisher_id,
                    ad_space_name,
                    publisher_earn,
                    creative_ref,
                    show_kind,
                    entertainment: env::current_account_id(),
                    entertainment_fee: fee,
                    status: String::from("signed")
                };
                
                self.presentations.insert(
                    &record_id,
                    &presentation,
                );
                
                Some(presentation)
            }
            None => None
        }
    }

    #[private]
    pub fn transfer_funds(&mut self, presentation_id: u64) -> bool {
        
        assert!(presentation_id > 0, "Abort. Presentation Id undefined");

        match self.presentations.get(&presentation_id) {
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
                self.presentations.insert(&presentation_id, &presentation);
                
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
}


