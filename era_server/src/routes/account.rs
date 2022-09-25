use actix_web::*;
use chrono::prelude::*;
use crate::{Result, State};
use crate::models::account::*;
use serde_json::json;

// i am lazy, pls fix soon
#[post("/api/oauth/token")]
pub async fn oauth_token(app: web::Data<State>) -> Result<impl Responder> {
    let soon = (Utc::now() + chrono::Duration::minutes(2147483647))
        .to_rfc3339_opts(SecondsFormat::Secs, true);
    let id = uuid::Uuid::new_v4().to_simple().to_string();
    
    Ok(HttpResponse::Ok().json(json!({
        "access_token": "era",
        "expires_in": 2147483647,
        "expires_at": soon,
        "token_type": "bearer",
        "refresh_token": "erarefresh",
        "refresh_expires": 2147483647,
        "refresh_expires_at": soon,
        "account_id": id.clone(),
        "client_id": "ec684b8c687f479fadea3cb2ad83f5c6",
        "internal_client": true,
        "client_service": "fortnite",
        "display_name": app.get_config()?.username,
        "app": "fortnite",
        "in_app_id": id
    })))
}

#[get("/api/public/account/{id}")]
pub async fn public_account(
    app: web::Data<State>,
    id: web::Path<String>
) -> Result<impl Responder> {
    let id = id.into_inner();
    let config = app.get_config()?;
    
    Ok(HttpResponse::Ok().json(json!({
        "id": id,
        "displayName": config.username,
        "name": "Project",
        "email": "era@erafn.glitch.me",
        "failedLoginAttempts": 0,
        "lastFailedLogin": "2021-01-22T23:00:00.000Z",
        "lastLogin": "2021-01-22T23:00:00.000Z",
        "numberOfDisplayNameChanges": 1,
        "ageGroup": "UNKNOWN",
        "headless": false,
        "country": "PL",
        "lastName": "Era",
        "preferredLanguage": "en",
        "lastDisplayNameChange": "2021-01-22T23:00:00.000Z",
        "canUpdateDisplayName": true,
        "tfaEnabled": true,
        "emailVerified": true,
        "minorVerified": false,
        "minorExpected": false,
        "minorStatus": "UNKNOWN"
    })))
}

#[get("/api/public/account")]
pub async fn public_account_query(
    app: web::Data<State>,
    query: web::Query<PublicAccount>
) -> Result<impl Responder> {
    let config = app.get_config()?;
    let query = query.into_inner();
    
    Ok(HttpResponse::Ok().json(json!([
        {
            "id": query.account_id,
            "displayName": config.username,
            "externalAuths": {}
        }
    ])))
}

#[get("/api/public/account/{i}/externalAuths")]
pub async fn external_auths() -> impl Responder {
    HttpResponse::Ok().json(json!([]))
}

#[get("/api/accounts/{i}/metadata")]
pub async fn accounts_metadata() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[delete("/api/oauth/sessions/kill")]
pub async fn kill_sessions() -> impl Responder {
    HttpResponse::NoContent()
}

#[delete("/api/oauth/sessions/kill/{i}")]
pub async fn kill_sessions_id() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/api/epicdomains/ssodomains")]
pub async fn ssodomains() -> impl Responder {
    HttpResponse::Ok().json(json!([]))
}