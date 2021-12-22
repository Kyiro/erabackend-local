use crate::attr;
use crate::models::app::{Config, CItem, CVariant, Locker};
use crate::utils::Build;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct QueryProfile {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipBattleRoyaleCustomization {
    pub item_to_slot: String,
    pub slot_name: String,
    #[serde(rename = "indexWithinSlot")]
    pub index: Option<i32>,
    #[serde(rename = "variantUpdates")]
    pub variants: Option<Vec<Variant>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetItemFavoriteStatusBatch {
    pub item_fav_status: Vec<bool>,
    pub item_ids: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCosmeticLockerSlot {
    pub locker_item: String,
    pub category: String,
    pub item_to_slot: String,
    pub slot_index: i32,
    #[serde(rename = "variantUpdates")]
    pub variants: Vec<Variant>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Variant {
    pub channel: String,
    pub active: String,
    pub owned: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SlotVariants {
    pub variants: Vec<SlotVariant>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SlotVariant {
    pub channel: String,
    pub active: String,
}

impl SlotVariants {
    pub fn from_variant(variants: Vec<Variant>) -> Self {
        Self {
            variants: variants
                .into_iter()
                .map(|v| SlotVariant {
                    channel: v.channel,
                    active: v.active
                })
                .collect()
        }
    }
}

impl Variant {
    pub fn new(updates: Vec<Self>, cvariants: Vec<CVariant>) -> Vec<Self> {
        let mut variants: Vec<Variant> = Vec::new();
        
        for v in cvariants {
            if &v.channel == "JerseyColor" {
                continue;
            }
            
            variants.push(Variant {
                channel: v.channel.clone(),
                // could be better but works :/
                active: match updates.iter().find(|u| u.channel == v.channel) {
                    Some(data) => data.active.clone(),
                    None => v.options.get(0).unwrap().clone(),
                },
                owned: v.options,
            });
        }
        
        for update in updates.into_iter() {
            if let None = variants.iter().find(|v| v.channel == update.channel) {
                variants.push(update);
            }
        }
        
        variants
    }
}

impl super::FullProfile {
    pub fn new_athena(account_id: String, cosmetics: Vec<CItem>, user: Config, build: Build) -> Self {
        let mut full = Self::new(account_id.clone(), String::from("athena"));
        let attributes = &mut full.profile.stats.attributes;
        
        // We do this for Enlightened styles (for e.g. golden skins in c2s2 or scratch corruption)
        let mut past_seasons = Vec::new();
        for i in 1..build.season {
            past_seasons.push(json!({
                "seasonNumber": i,
                "numWins": 999,
                "numHighBracket": 999,
                "numLowBracket": 999,
                "seasonXp": 999999,
                "seasonLevel": 999,
                "bookXp": 0,
                "bookLevel": 100,
                "purchasedVIP": true
            }));
        }
        
        attr!(attributes, "past_seasons" => past_seasons);
        attr!(attributes, "season_match_boost", 120);
        attr!(attributes, "mfa_reward_claimed", true);
        attr!(attributes, "rested_xp_overflow", 0);
        attr!(attributes, "quest_manager" => {
            "dailyLoginInterval": "2021-06-24T11:24:14.414Z",
            "dailyQuestRerolls": 1
        });
        attr!(attributes, "book_level", 100);
        attr!(attributes, "season_num", build.season);
        attr!(attributes, "book_xp", 999999);
        attr!(attributes, "permissions" => []);
        attr!(attributes, "season" => {
            "numWins": 0,
            "numHighBracket": 0,
            "numLowBracket": 0
        });
        attr!(attributes, "battlestars", 9999);
        attr!(attributes, "vote_data" => {});
        attr!(attributes, "book_purchased", true);
        attr!(attributes, "lifetime_wins", 999);
        attr!(attributes, "party_assist_quest", "");
        attr!(attributes, "purchased_battle_pass_tier_offers" => {});
        attr!(attributes, "rested_xp_exchange", 1);
        attr!(attributes, "level", match build.season {
            11 | 12 => 350,
            _ => 100
        });
        attr!(attributes, "xp_overflow", 0);
        attr!(attributes, "rested_xp", 120);
        attr!(attributes, "rested_xp_mult", 120);
        attr!(attributes, "account_level", 120);
        attr!(attributes, "competitive_identity", 120);
        attr!(attributes, "inventory_limit_bonus", 120);
        attr!(attributes, "daily_rewards", 120);
        attr!(attributes, "xp", 9999999);
        attr!(attributes, "season_friend_match_boost", 40);
        // cosmetics
        attr!(attributes, "favorite_character", user.locker.character.items[0]);
        attr!(attributes, "favorite_backpack", user.locker.backpack.items[0]);
        attr!(attributes, "favorite_pickaxe", user.locker.pickaxe.items[0]);
        attr!(attributes, "favorite_glider", user.locker.glider.items[0]);
        attr!(attributes, "favorite_skydivecontrail", user.locker.contrail.items[0]);
        attr!(attributes, "favorite_musicpack", user.locker.music_pack.items[0]);
        attr!(attributes, "favorite_loadingscreen", user.locker.loading.items[0]);
        attr!(attributes, "favorite_dance", user.locker.dance.items);
        attr!(attributes, "favorite_itemwraps", user.locker.item_wrap.items);
        // unused cosmetics
        attr!(attributes, "favorite_callingcard", "");
        attr!(attributes, "favorite_consumableemote", "");
        attr!(attributes, "favorite_spray" => []);
        attr!(attributes, "favorite_hat", "");
        attr!(attributes, "favorite_battlebus", "");
        attr!(attributes, "favorite_mapmarker", "");
        attr!(attributes, "favorite_vehicledeco", "");
        attr!(attributes, "favorite_victorypose", "");
        // loadouts (s12+)
        attr!(attributes, "active_loadout_index", 0);
        attr!(attributes, "use_random_loadout", false);
        attr!(attributes, "last_applied_loadout", "eraloadout");
        attr!(attributes, "loadouts" => []);
        
        for item in cosmetics {
            let template = format!("{}:{}", item.item_type, item.id.to_lowercase());
            
            full.add_cosmetic(
                template.clone(),
                item,
                user.favourites.contains(&template),
                user.variants.get(&template).unwrap_or(&Vec::new()).clone()
            );
        }
        
        full.add_loadout(String::from("eraloadout"), user.locker);
        
        full
    }
    
    pub fn add_cosmetic(
        &mut self,
        template: String,
        item: CItem,
        favourite: bool,
        variants: Vec<Variant>
    ) -> &Value {
        self.profile.items.insert(template.clone(), json!({
            "templateId": template,
            "attributes": {
                "max_level_bonus": 0 as usize,
                "level": 1 as usize,
                "item_seen": true,
                "xp": 0 as usize,
                "variants": Variant::new(variants, item.variants),
                "favorite": favourite
            },
            "quantity": 1 as usize
        }));
        
        self.profile.items.get_mut(&template).unwrap()
    }
    
    pub fn add_loadout(&mut self, name: String, locker: Locker) -> &Value {
        self.profile.items.insert(name.clone(), generate_loadout(locker));
        
        self.profile.stats.attributes.get_mut("loadouts")
            .unwrap()
            .as_array_mut()
            .unwrap()
            .push(json!(name));
        
        self.profile.items.get_mut(&name).unwrap()
    }
}

pub fn generate_loadout(locker: Locker) -> Value {
    json!({
        "templateId": "CosmeticLocker:cosmeticlocker_athena",
        "attributes": {
            "locker_slots_data": {
                "slots": {
                    "SkyDiveContrail": {
                        "items": locker.contrail.items,
                        "activeVariants": locker.contrail.variants
                    },
                    "MusicPack": {
                        "items": locker.music_pack.items,
                        "activeVariants": locker.music_pack.variants
                    },
                    "Character": {
                        "items": locker.character.items,
                        "activeVariants": locker.character.variants
                    },
                    "Backpack": {
                        "items": locker.backpack.items,
                        "activeVariants": locker.backpack.variants
                    },
                    "Glider": {
                        "items": locker.glider.items,
                        "activeVariants": locker.glider.variants
                    },
                    "Pickaxe": {
                        "items": locker.pickaxe.items,
                        "activeVariants": locker.pickaxe.variants
                    },
                    "ItemWrap": {
                        "items": locker.item_wrap.items,
                        "activeVariants": locker.item_wrap.variants
                    },
                    "LoadingScreen": {
                        "items": locker.loading.items,
                        "activeVariants": locker.loading.variants
                    },
                    "Dance": {
                        "items": locker.dance.items,
                        "activeVariants": locker.dance.variants
                    }
                }
            },
            "use_count": 1 as usize,
            "banner_icon_template": "",
            "banner_color_template": "",
            "locker_name": locker.name.clone(),
            "item_seen": true,
            "favorite": false
        },
        "quantity": 1 as usize
    })
}