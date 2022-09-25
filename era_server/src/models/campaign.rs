use crate::attr;
use chrono::prelude::*;
use serde_json::json;

impl super::FullProfile {
    pub fn new_campaign(account_id: String) -> Self {
        let mut full = Self::new(String::from("campaign"), account_id);
        
        full.fill_in_campaign();
        
        full
    }
    
    pub fn fill_in_campaign(&mut self) {
        let attributes = &mut self.profile.stats.attributes;
        
        attr!(attributes, "daily_rewards" => {
            "nextDefaultReward": 365,
            "totalDaysLoggedIn": 365,
            "lastClaimDate": Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
        });
        
        self.complete_onboarding();
    }
    
    pub fn grant_campaign(&mut self) {
        self.profile.items.insert(String::from("Token:campaignaccess"), json!({
            "templateId": "Token:campaignaccess",
            "attributes": {
                "max_level_bonus": 0,
                "level": 1,
                "item_seen": false,
                "xp": 0,
                "favorite": false
            },
            "quantity": 1
        }));
    }
    
    pub fn complete_onboarding(&mut self) {
        self.profile.items.insert(String::from("Quest:homebaseonboarding"), json!({
            "templateId": "Quest:homebaseonboarding",
            "attributes": {
                "creation_time": "min",
                "level": -1,
                "completion_hbonboarding_completezone": 1,
                "item_seen": false,
                "playlists": [],
                "sent_new_notification": true,
                "challenge_bundle_id": "",
                "xp_reward_scalar": 1,
                "challenge_linked_quest_given": "",
                "quest_pool": "",
                "quest_state": "Claimed",
                "bucket": "",
                "last_state_change_time": "2019-12-18T19:12:36.774Z",
                "challenge_linked_quest_parent": "",
                "max_level_bonus": 0,
                "completion_hbonboarding_namehomebase": 1,
                "completion_hbonboarding_watchsatellitecine": 1,
                "xp": 0,
                "quest_rarity": "uncommon",
                "favorite": false
            },
            "quantity": 1
        }));
    }
}