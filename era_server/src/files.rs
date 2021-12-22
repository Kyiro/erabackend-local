use crate::CENTRAL;
use crate::models::app::CItem;
use serde_json::{json, Value};

pub async fn cosmetics(offline: bool) -> Vec<CItem> {
    let fallback = serde_json::from_str(
        include_str!("./../resources/cosmetics.json")
    ).unwrap_or(Vec::new());
    
    if offline {
        return fallback
    }
    
    if let Ok(req) = reqwest::get(CENTRAL.to_owned() + "/public/cosmetics.json").await {
        req.json().await.unwrap_or(fallback)
    } else {
        fallback
    }
}

pub async fn fortnite_game(offline: bool) -> Value {
    let fallback = serde_json::from_str(
        include_str!("./../resources/fortnite-game.json")
    ).unwrap_or(json!({}));
    
    if offline {
        return fallback
    }
    
    if let Ok(req) = reqwest::get(CENTRAL.to_owned() + "/public/fortnite-game.json").await {
        req.json().await.unwrap_or(fallback)
    } else {
        fallback
    }
}

pub async fn keychain(offline: bool) -> Vec<String> {
    let fallback = serde_json::from_str(
        include_str!("./../resources/keychain.json")
    ).unwrap_or(Vec::new());
    
    if offline {
        return fallback
    }
    
    if let Ok(req) = reqwest::get(CENTRAL.to_owned() + "/public/keychain.json").await {
        req.json().await.unwrap_or(fallback)
    } else {
        fallback
    }
}

pub async fn status(offline: bool) -> Value {
    let fallback = serde_json::from_str(
        include_str!("./../resources/status.json")
    ).unwrap_or(json!({}));
    
    if offline {
        return fallback
    }
    
    if let Ok(req) = reqwest::get(CENTRAL.to_owned() + "/public/status.json").await {
        req.json().await.unwrap_or(fallback)
    } else {
        fallback
    }
}