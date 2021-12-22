use crate::attr;
use serde_json::json;

impl super::FullProfile {
    pub fn new_common_core(account_id: String) -> Self {
        let mut full = Self::new(String::from("common_core"), account_id);
        let attributes = &mut full.profile.stats.attributes;
        
        attr!(attributes, "survey_data" => {});
        attr!(attributes, "personal_offers" => {});
        attr!(attributes, "intro_game_played", false);
        attr!(attributes, "import_friends_claimed" => {});
        attr!(attributes, "mtx_purchase_history" => {});
        attr!(attributes, "undo_cooldowns" => []);
        attr!(attributes, "mtx_affiliate_set_time", "");
        attr!(attributes, "inventory_limit_bonus", 0);
        attr!(attributes, "current_mtx_platform", "EpicPC");
        attr!(attributes, "mtx_affiliate", 0);
        attr!(attributes, "weekly_purchases" => {});
        attr!(attributes, "daily_purchases" => {});
        attr!(attributes, "ban_history" => {});
        attr!(attributes, "in_app_purchases" => {});
        attr!(attributes, "permissions" => []);
        attr!(attributes, "undo_timeout", "min");
        attr!(attributes, "monthly_purchases" => {});
        attr!(attributes, "allowed_to_send_gifts", true);
        attr!(attributes, "mfa_enabled", false);
        attr!(attributes, "allowed_to_receive_gifts", true);
        attr!(attributes, "gift_history" => {});
        
        full.set_vbucks(13500);
        
        full
    }
    
    pub fn set_vbucks(&mut self, amount: i32) {
        self.profile.items.insert(String::from("Currency:MtxComplimentary"), json!({
            "templateId": "Currency:MtxComplimentary",
            "attributes": {
                "platform": "Shared"
            },
            "quantity": amount
        }));
    }
}