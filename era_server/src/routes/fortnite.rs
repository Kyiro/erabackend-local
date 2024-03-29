use actix_web::*;
use chrono::prelude::*;
use crate::State;
use crate::utils::{get_build, Build};
use serde_json::json;

#[get("/api/v2/versioncheck/{i}")]
pub async fn version_check_v2() -> impl Responder {
    HttpResponse::NoContent().json(json!({
        "type": "NO_UPDATE"
    }))
}

#[get("/api/versioncheck")]
pub async fn version_check() -> impl Responder {
    HttpResponse::NoContent().json(json!({
        "type": "NO_UPDATE"
    }))
}

#[get("/api/game/v2/enabled_features")]
pub async fn enabled_features() -> impl Responder {
    HttpResponse::Ok().json(json!([]))
}

#[get("/api/receipts/v1/account/{i}/receipts")]
pub async fn receipts() -> impl Responder {
    HttpResponse::Ok().json(json!([]))
}

#[post("/api/game/v2/tryPlayOnPlatform/account/{i}")]
pub async fn play_on_platform() -> impl Responder {
    HttpResponse::Ok().body("true")
}

#[get("/api/matchmaking/session/findPlayer/{i}")]
pub async fn find_player() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/api/game/v2/world/info")]
pub async fn world_info(app: web::Data<State>) -> impl Responder {
    HttpResponse::Ok().json(app.world_info.clone())
}

#[get("/api/storefront/v2/keychain")]
pub async fn keychain(app: web::Data<State>) -> impl Responder {
    HttpResponse::Ok().json(app.keychain.clone())
}

#[get("/api/version")]
pub async fn fortnite_version() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "app": "fortnite",
        "serverDate": Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        "overridePropertiesVersion": "unknown",
        "cln": "2870186",
        "build": "1",
        "moduleName": "Fortnite-Core",
        "buildDate": "2016-02-17T10:16:51.000Z",
        "version": "4.12.0-2870186+++Fortnite+Release-Live",
        "branch": "++Fortnite+Release-Live",
        "modules": {}
    }))
}

#[get("/api/game/v2/twitch/{i}")]
pub async fn twitch() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[get("/api/calendar/v1/timeline")]
pub async fn timeline(req: HttpRequest) -> impl Responder {
    let build = get_build(&req).unwrap_or(Build::default());
    let day = (Utc::now() + chrono::Duration::days(1)).to_rfc3339_opts(SecondsFormat::Secs, true);

    HttpResponse::Ok().json(json!({
      "channels": {
        "client-events": {
          "states": [
            {
              "validFrom": "2000-01-01T10:00:00Z",
              "activeEvents": [
                {
                  "eventType": format!("EventFlag.Season{}", build.season),
                  "activeUntil": "9999-01-01T22:28:47.830Z",
                  "activeSince": "2000-01-01T10:00:00Z"
                },
                {
                  "eventType": match build.season {
                      2 => String::from("EventFlag.LobbyWinterDecor"),
                      _ => if build.patch == Some(6.21) {
                        String::from("EventFlag.LobbySeason6Halloween")
                      } else {
                          format!("EventFlag.LobbySeason{}", build.season)
                        }
                  },
                  "activeUntil": "9999-01-01T22:28:47.830Z",
                  "activeSince": "2000-01-01T10:00:00Z"
                }
              ],
              "state": {
                "activeStorefronts": [],
                "eventNamedWeights": {},
                "seasonNumber": build.season,
                "seasonTemplateId": format!("AthenaSeason:athenaseason{}", build.season),
                "matchXpBonusPoints": 0 as usize,
                "seasonBegin": "2000-01-01T10:00:00Z",
                "seasonEnd": day,
                "seasonDisplayedEnd": day,
                "weeklyStoreEnd": day,
                "stwEventStoreEnd": day,
                "stwWeeklyStoreEnd": day,
                "dailyStoreEnd": day
              }
            }
          ],
          "cacheExpire": "9999-01-01T22:28:47.830Z"
        }
      },
      "eventsTimeOffsetHrs": 0 as usize,
      "cacheIntervalMins": 9999 as usize,
      "currentTime": Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
    }))
}

#[get("/api/storefront/v2/catalog")]
pub async fn catalog() -> impl Responder {
    HttpResponse::NotFound()
}