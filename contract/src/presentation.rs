    use crate::*;

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

#[near_bindgen]
impl MetaAdsContract {
    
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

        if let Some(creative) = self.creatives.get(&creative_id) {

            if let Some(adspot) = self.ad_spots.get(&adspot_id) {

                let deposit: Balance = env::attached_deposit();
                assert!(deposit >= adspot.price, "Deposit is too small. Attached: {}, Required: {}", deposit, adspot.price);

                let advertiser_account_id = env::predecessor_account_id();
                let owner_account_id = creative.owner_account_id.clone();
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
                    publisher_account_id: adspot.owner_account_id.clone(),
                    ad_spot_name: adspot.name.clone(),
                    publisher_earn: adspot.publisher_earn.clone(),
                    show_kind: adspot.show_kind.clone(),
                    entertainment: env::current_account_id(),
                    entertainment_fee: fee,
                    status: String::from("signed")
                };

                assert!(
                    self.presentations.insert(&playback_id, &presentation).is_none(),
                    "Presentation already exists"
                );
                
                Some(presentation)

            } else {
                near_sdk::env::panic(b"Ad Spot not found");
            }    
        } else {
            near_sdk::env::panic(b"Creative not found");
        }
    }

    pub fn fetch_all_presentations(&self) -> HashMap<u64, Presentation> {
        self.presentations.iter().collect()
    }

    pub fn fetch_presentation_by_id(&self, id: u64) -> Presentation {
        self.presentations.get(&id).expect("Presentation not found")
    }
}  