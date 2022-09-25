use crate::attr;

impl super::FullProfile {
    pub fn new_profile0(account_id: String) -> Self {
        let mut full = Self::new(String::from("profile0"), account_id);
        let attributes = &mut full.profile.stats.attributes;
        
        attr!(attributes, "node_costs" => {});
        attr!(attributes, "mission_alert_redemption_record" => {});
        attr!(attributes, "twitch" => {});
        attr!(attributes, "client_settings" => {});
        attr!(attributes, "level", 0);
        attr!(attributes, "named_counters" => {
            "SubGameSelectCount_Campaign": {
                "current_count": 0
            },
            "SubGameSelectCount_Athena": {
                "current_count": 0
            }
        });
        attr!(attributes, "default_hero_squad_id", "");
        attr!(attributes, "collection_book" => {});
        attr!(attributes, "quest_manager" => {
            "dailyLoginInterval": "2017-01-01T01:00:00.602Z",
            "dailyQuestRerolls": 1
        });
        attr!(attributes, "bans" => {});
        attr!(attributes, "gameplay_stats" => []);
        attr!(attributes, "inventory_limit_bonus", 0);
        attr!(attributes, "current_mtx_platform", "Epic");
        attr!(attributes, "weekly_purchases" => {});
        attr!(attributes, "daily_purchases" => {});
        attr!(attributes, "mode_loadouts" => []);
        attr!(attributes, "in_app_purchases" => {});
        attr!(attributes, "daily_rewards" => {});
        attr!(attributes, "monthly_purchases" => {});
        attr!(attributes, "xp", 0);
        attr!(attributes, "homebase" => {
            "townName": "ProjectEra",
            "bannerIconId": "",
            "bannerColorId": "",
            "flagPattern": -1,
            "flagColor": -1
        });
        attr!(attributes, "packs_granted", 0);
        
        full.set_vbucks(13500);
        full.grant_campaign();
        full.fill_in_campaign();
        
        full
    }
}