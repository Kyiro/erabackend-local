use actix_web::*;
use crate::{Result, State};
use crate::models::*;
use crate::utils::{Build, get_build};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    #[serde(rename = "profileId")]
    pub profile_id: String,
    pub rvn: i32,
}

#[post("/api/game/v2/profile/{id}/client/QueryProfile")]
pub async fn query_profile(
    app: web::Data<State>,
    _: web::Json<athena::QueryProfile>,
    query: web::Query<Query>,
    id: web::Path<String>,
    req: HttpRequest
) -> impl Responder {
    let query = query.into_inner();
    let account_id = id.into_inner();
    let build = get_build(&req).unwrap_or(Build::default());
    
    log::debug!(
        "QueryProfile (profileId: {}, rvn: {}, id: {})",
        query.profile_id,
        query.rvn,
        account_id
    );
    
    match query.profile_id.as_str() {
        "athena" => HttpResponse::Ok().json(Profile::new(
            String::from("athena"),
            vec![ProfileChanges::Full(FullProfile::new_athena(
                account_id,
                app.cosmetics.clone(),
                app.get_config().unwrap(),
                build
            ))],
            None
        )),
        "campaign" => HttpResponse::Ok().json(Profile::new(
            String::from("campaign"),
            vec![ProfileChanges::Full(FullProfile::new_campaign(account_id))],
            None
        )),
        "profile0" => HttpResponse::Ok().json(Profile::new(
            query.profile_id,
            vec![ProfileChanges::Full(FullProfile::new_profile0(account_id))],
            None
        )),
        "common_core" => HttpResponse::Ok().json(Profile::new(
            query.profile_id,
            vec![ProfileChanges::Full(FullProfile::new_common_core(account_id))],
            None,
        )),
        "common_public" => HttpResponse::Ok().json(Profile::new(
            query.profile_id,
            vec![ProfileChanges::Full(FullProfile::new_common_public(account_id))],
            None,
        )),
        _ => HttpResponse::Ok().json(Profile::new(
            query.profile_id.clone(),
            vec![ProfileChanges::Full(FullProfile::new(query.profile_id, account_id))],
            Some(query.rvn)
        ))
    }
}

#[post("/api/game/v2/profile/{id}/client/EquipBattleRoyaleCustomization")]
pub async fn equip_battle_royale_customization(
    app: web::Data<State>,
    body: web::Json<athena::EquipBattleRoyaleCustomization>,
    query: web::Query<Query>,
    id: web::Path<String>
) -> Result<impl Responder> {
    let body = body.into_inner();
    let _id = id.into_inner();
    let query = query.into_inner();
    let idx = body.index.unwrap_or(0);
    let favorite_slot = {
        let mut slot = format!("favorite_{}", body.slot_name.to_lowercase());
        
        if slot == "favorite_itemwrap" {
            slot = "favorite_itemwraps".to_string();
        }
        
        slot
    };
    let item = app::CItem::from_string(body.item_to_slot.clone());
    let item = app.get_item(item.clone()).unwrap_or(item);
    
    log::debug!("{:?}", body.clone());
    log::debug!("Equipping {} with {} variant(s)", item.id.clone(), item.variants.len());
    
    let mut config = app.get_config()?;
    let mut changes = Vec::new();
    
    if let Some(variants) = body.variants {
        if variants.len() != 0 {
            config.variants.insert(
                body.item_to_slot.clone(),
                variants.clone()
            );
            
            changes.push(ProfileChanges::Changed(AttrChanged::new(
                body.item_to_slot.clone(),
                String::from("variants"),
                athena::Variant::new(variants, item.variants)
            )));
        }
    }
    
    if let Some((slots, _)) = config.locker.get_mut(body.slot_name.clone()) {
        if idx < 0 {
            for slot in slots.iter_mut() {
                *slot = body.item_to_slot.clone();
            }
        } else {
            slots[idx as usize] = body.item_to_slot.clone();
        }
        
        if slots.len() == 1 {
            changes.push(ProfileChanges::Stat(StatModified::new(&favorite_slot, body.item_to_slot)));
        } else {
            changes.push(ProfileChanges::Stat(StatModified::new(&favorite_slot, slots)));
        }
    }
    
    app.save_config(&config)?;
    
    Ok(HttpResponse::Ok().json(Profile::new(
        query.profile_id,
        changes,
        Some(query.rvn)
    )))
}

#[post("/api/game/v2/profile/{id}/client/SetItemFavoriteStatusBatch")]
pub async fn set_item_favorite_status_batch(
    app: web::Data<State>,
    body: web::Json<athena::SetItemFavoriteStatusBatch>,
    query: web::Query<Query>
) -> Result<impl Responder> {
    let body = body.into_inner();
    let query = query.into_inner();
    
    let mut config = app.get_config()?;
    let mut changes = Vec::new();
    
    for idx in 0..body.item_fav_status.len() {
        let (status, id) = (body.item_fav_status[idx], body.item_ids[idx].clone());
        
        log::debug!("Favourite {} to {}", id, status);
        
        if status == true {
            config.favourites.push(id.clone());
        } else {
            config.favourites = config.favourites
                .into_iter()
                .filter(|i| **i != id)
                .collect()
        }
        
        changes.push(ProfileChanges::Changed(AttrChanged::new(
            id,
            String::from("favorite"),
            status
        )));
    }
    
    app.save_config(&config)?;
    
    Ok(HttpResponse::Ok().json(Profile::new(
        query.profile_id,
        changes,
        Some(query.rvn)
    )))
}

#[post("/api/game/v2/profile/{id}/client/SetCosmeticLockerSlot")]
pub async fn set_cosmetic_locker_slot(
    app: web::Data<State>,
    body: web::Json<athena::SetCosmeticLockerSlot>,
    query: web::Query<Query>
) -> Result<impl Responder> {
    let body = body.into_inner();
    let query = query.into_inner();
    
    let item = app::CItem::from_string(body.item_to_slot.clone());
    let item = app.get_item(item.clone()).unwrap_or(item);
    
    let mut config = app.get_config()?;
    let mut changes = Vec::new();
    
    let gen_variants = athena::Variant::new(body.variants.clone(), item.variants.clone());
    
    if body.variants.len() != 0 {
        config.variants.insert(
            body.item_to_slot.clone(),
            body.variants.clone()
        );
        
        changes.push(ProfileChanges::Changed(AttrChanged::new(
            body.item_to_slot.clone(),
            String::from("variants"),
            gen_variants.clone()
        )));
    }
    
    if let Some((slots, slot_variants)) = config.locker.get_mut(body.category) {
        if body.slot_index < 0 {
            for i in 0..slots.len() {
                let slot = &mut slots[i];
                let variants = &mut slot_variants[i];
                
                *slot = body.item_to_slot.clone();
                if body.variants.len() == 0 {
                    *variants = None;
                } else {
                    *variants = Some(athena::SlotVariants::from_variant(
                        gen_variants.clone()
                    ));
                }
            }
        } else {
            slots[body.slot_index as usize] = body.item_to_slot.clone();
            slot_variants[body.slot_index as usize] = Some(athena::SlotVariants::from_variant(
                gen_variants.clone()
            ));
        }
        
        changes.push(ProfileChanges::Changed(AttrChanged::new(
            body.locker_item.clone(),
            String::from("locker_slots_data"),
            athena::generate_loadout(
                config.locker.clone()
            )["attributes"]["locker_slots_data"].clone()
        )))
    }
    
    app.save_config(&config)?;
    
    Ok(HttpResponse::Ok().json(Profile::new(
        query.profile_id,
        changes,
        Some(query.rvn)
    )))
}

#[post("/api/game/v2/profile/{id}/client/{action}")]
pub async fn other(query: web::Query<Query>) -> impl Responder {
    let query = query.into_inner();
    
    HttpResponse::Ok().json(Profile::new(
        query.profile_id,
        Vec::new(),
        Some(query.rvn)
    ))
}