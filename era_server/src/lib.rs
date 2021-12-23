#![recursion_limit = "512"]
use actix_web::*;
use std::error::Error as StdError;

pub mod files;
pub mod models;
pub mod routes;
pub mod utils;

pub const CENTRAL: &'static str = "https://eracentral.kyiro.repl.co";
pub type State = models::app::State;
pub type Result<T, E = Box<dyn StdError>> = std::result::Result<T, E>;

pub fn init(port: u16) -> std::io::Result<()> {
    utils::init_logger();
    
    rt::System::new().block_on(async move {
        start(port).await
    })
}

pub async fn start(port: u16) -> std::io::Result<()> {
    log::info!("Initializing EraAPI");
    let state = web::Data::new(State::new().await);
    
    HttpServer::new(move || {
        App::new()
        .service(routes::other::blocklist)
        .service(routes::other::datarouter)
        .service(routes::other::eulatracking)
        .service(routes::other::fortnite_game)
        .service(routes::other::friends)
        .service(routes::other::party_user)
        .service(routes::other::recent_players)
        .service(routes::other::settings)
        .service(routes::other::status)
        .service(routes::other::waitingroom)
        .service(
            web::scope("/account")
            .service(routes::account::accounts_metadata)
            .service(routes::account::external_auths)
            .service(routes::account::kill_sessions)
            .service(routes::account::kill_sessions_id)
            .service(routes::account::oauth_token)
            .service(routes::account::public_account)
            .service(routes::account::public_account_query)
            .service(routes::account::ssodomains)
        )
        .service(
            web::scope("/fortnite")
            .service(routes::fortnite::catalog)
            .service(routes::fortnite::enabled_features)
            .service(routes::fortnite::find_player)
            .service(routes::fortnite::fortnite_version)
            .service(routes::fortnite::keychain)
            .service(routes::fortnite::play_on_platform)
            .service(routes::fortnite::receipts)
            .service(routes::fortnite::timeline)
            .service(routes::fortnite::twitch)
            .service(routes::fortnite::version_check)
            .service(routes::fortnite::version_check_v2)
            .service(routes::fortnite::world_info)
            .service(routes::cloudstorage::system_config)
            .service(routes::cloudstorage::system_get_file)
            .service(routes::cloudstorage::system_list)
            .service(routes::cloudstorage::user_get_file)
            .service(routes::cloudstorage::user_list)
            .service(routes::cloudstorage::user_put_file)
            .service(routes::mcp::equip_battle_royale_customization)
            .service(routes::mcp::query_profile)
            .service(routes::mcp::set_item_favorite_status_batch)
            .service(routes::mcp::set_cosmetic_locker_slot)
            .service(routes::mcp::other)
        )
        .app_data(state.clone())
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}