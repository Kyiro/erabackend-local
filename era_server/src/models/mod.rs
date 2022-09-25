use chrono::prelude::*;
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::HashMap;

pub mod account;
pub mod app;
pub mod athena;
pub mod campaign;
pub mod cloudstorage;
pub mod common_core;
pub mod common_public;
pub mod profile0;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub profile_revision: i32,
    pub profile_id: String,
    pub profile_changes_base_revision: i32,
    pub profile_changes: Vec<ProfileChanges>,
    pub profile_command_revision: i32,
    pub server_time: String,
    pub response_version: i32,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ProfileChanges {
    Full(FullProfile),
    Stat(StatModified),
    Changed(AttrChanged)
}

#[derive(Serialize)]
pub struct StatModified {
    #[serde(rename = "changeType")]
    pub change_type: String,
    pub name: String,
    pub value: Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttrChanged {
    pub change_type: String,
    pub item_id: String,
    pub attribute_name: String,
    pub attribute_value: Value,
}

impl StatModified {
    pub fn new<T: Serialize>(name: &str, value: T) -> Self {
        Self {
            change_type: String::from("statModified"),
            name: name.to_string(),
            value: json!(value)
        }
    }
}

impl AttrChanged {
    pub fn new<T: Serialize>(item_id: String, name: String, value: T) -> Self {
        Self {
            change_type: String::from("itemAttrChanged"),
            item_id,
            attribute_name: name,
            attribute_value: json!(value)
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullProfile {
    pub change_type: String,
    pub profile: FullProfileInner,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullProfileInner {
    #[serde(rename = "_id")]
    pub _id: String,
    pub created: String,
    pub updated: String,
    pub rvn: i32,
    pub wipe_number: i32,
    pub account_id: String,
    pub profile_id: String,
    pub version: String,
    pub items: HashMap<String, Value>,
    pub stats: Stats,
    pub command_revision: i32,
}

#[derive(Serialize)]
pub struct Stats {
    pub attributes: HashMap<String, Value>,
}

impl Profile {
    pub fn new(profile_id: String, changes: Vec<ProfileChanges>, rvn: Option<i32>) -> Self {
        Self {
            profile_revision: rvn.unwrap_or(1) + 1,
            profile_id: profile_id,
            profile_changes_base_revision: rvn.unwrap_or(2),
            profile_changes: changes,
            profile_command_revision: rvn.unwrap_or(1) + 1,
            server_time: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            response_version: 1
        }
    }
}

impl FullProfile {
    pub fn new(profile_id: String, account_id: String) -> Self {
        Self {
            change_type: String::from("fullProfileUpdate"),
            profile: FullProfileInner {
                _id: account_id.clone(),
                created: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
                updated:  Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
                rvn: 1,
                wipe_number: 1,
                account_id,
                profile_id,
                version: String::from("era-backend"),
                items: HashMap::new(),
                stats: Stats {
                    attributes: HashMap::new()
                },
                command_revision: 1
            }
        }
    }
}