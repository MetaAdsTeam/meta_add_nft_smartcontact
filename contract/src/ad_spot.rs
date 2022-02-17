use crate::*;

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
impl MetaAdsContract {

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
            price: price * SPOT_NEAR,
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

    pub fn fetch_all_adspots(&self) -> HashMap<u64, AdSpot> {
        self.ad_spots.iter().collect()
    }

    pub fn fetch_adspot_by_id(&self, id: u64) -> AdSpot {
        self.ad_spots.get(&id).expect("AdSpot not found")
    }
}