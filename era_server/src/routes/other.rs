use actix_web::*;
use crate::State;
use crate::utils::{get_build, Build};
use serde_json::json;

#[get("/waitingroom/api/waitingroom")]
pub async fn waitingroom() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/party/api/v1/Fortnite/user/{u}")]
pub async fn party_user() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "current": [],
        "pending": [],
        "invites": [],
        "pings": []
    }))
}

#[get("/friends/api/public/friends/{i}")]
pub async fn friends() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/friends/api/public/list/fortnite/{i}/recentPlayers")]
pub async fn recent_players() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/friends/api/public/blocklist/{i}")]
pub async fn blocklist() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "blockedUsers": []
    }))
}

#[get("/friends/api/v1/{i}/settings")]
pub async fn settings() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[post("/datarouter/api/v1/public/data")]
pub async fn datarouter() -> impl Responder {
    HttpResponse::NoContent().json(json!({}))
}

#[get("/eulatracking/api/shared/agreements/fn")]
pub async fn eulatracking() -> impl Responder {
    HttpResponse::NoContent().json(json!({}))
}

#[get("/lightswitch/api/service/{service}/status")]
pub async fn status(
    app: web::Data<crate::State>,
    service: web::Path<String>
) -> impl Responder {
    let service = service.into_inner();
    
    let mut data = app.status.clone();
    
    if service == "bulk" {
        data = json!([data]);
    } else {
        data["serviceInstanceId"] = json!(service.to_lowercase());
    }
    
    HttpResponse::Ok().json(data)
}

#[get("/content/api/pages/fortnite-game")]
pub async fn fortnite_game(
    app: web::Data<State>,
    req: HttpRequest
) -> impl Responder {
    let build = get_build(&req).unwrap_or(Build::default());
    let mut game = app.game.clone();
    
    game["dynamicbackgrounds"] = json!({
        "jcr:isCheckedOut": true,
        "backgrounds": {
            "backgrounds": [
                {
                    "stage": format!("season{}", match build.season {
                        10 => String::from("x"),
                        season => season.to_string()
                    }),
                    "_type": "DynamicBackground",
                    "key": "lobby"
                }
            ],
            "_type": "DynamicBackgroundList"
        },
        "_title": "dynamicbackgrounds",
        "_noIndex": false,
        "jcr:baseVersion": "a7ca237317f1e734b3b9d6-8d7b-42cd-bcf1-19decf60552a",
        "_activeDate": "2018-08-06T19:00:26.217Z",
        "lastModified": "2018-08-06T19:00:26.217Z",
        "_locale": "en-US"
    });
    
    HttpResponse::Ok().json(game)
}