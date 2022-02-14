use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Creative {
    pub creative_id: u64,
    pub name: String,
    pub content: String,
    pub nft_cid: Option<String>,
    pub owner_account_id: AccountId,
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

    pub fn fetch_all_creatives(&self) -> HashMap<u64, Creative> {
        self.creatives.iter().collect()
    }

    pub fn fetch_creative_by_id(&self, id: u64) -> Creative {
        self.creatives.get(&id).expect("Creative not found")
    }
}    