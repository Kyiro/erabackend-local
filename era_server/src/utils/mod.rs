use actix_web::*;
use crate::{CENTRAL};
use regex::Regex;
use std::net::{SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr, TcpListener, ToSocketAddrs};

// i hate this
#[macro_export]
macro_rules! attr {
    ($list:expr, $key:tt => $val:tt) => {
        $list.insert(String::from($key), serde_json::json!($val))
    };
    
    ($list:expr, $key:tt, $val:expr) => {
        $list.insert(String::from($key), serde_json::json!($val))
    };
}

#[derive(Debug)]
pub struct Build {
    pub season: usize,
    pub patch: Option<f32>,
    pub netcl: usize,
}

impl Default for Build {
    fn default() -> Self {
        Self {
            season: 1,
            patch: None,
            netcl: 3709086
        }
    }
}

pub fn init_logger() {
    if let Err(_) = std::env::var("RUST_LOG") {
        #[cfg(debug_assertions)]
        std::env::set_var("RUST_LOG", "debug");
        #[cfg(not(debug_assertions))]
        std::env::set_var("RUST_LOG", "info");
    }
    
    env_logger::init();
}

fn test_bind_tcp<A: ToSocketAddrs>(addr: A) -> Option<u16> {
    Some(TcpListener::bind(addr).ok()?.local_addr().ok()?.port())
}

pub fn is_free(port: u16) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, port, 0, 0);
    
    test_bind_tcp(ipv4).is_some() && test_bind_tcp(ipv6).is_some()
}

pub fn get_build(req: &HttpRequest) -> Option<Build> {
    let useragent = req.headers().get("User-Agent")?.to_str().ok()?;
    
    let regex = Regex::new(r"[^\w=](\d{1}|\d{2}).(\d{2}|\d{1}).*-(\d{8}|\d{7})|-(\d{7})").ok()?;
    let captures = regex.captures(useragent)?;
    
    let netcl: usize = match captures.get(3) {
        Some(netcl) => netcl,
        None => captures.get(4)?,
    }
    .as_str()
    .parse()
    .ok()?;

    let season = if netcl < 3807424 {
        1
    } else if netcl < 3901517 {
        2
    } else {
        captures.get(1)?.as_str().parse().ok()?
    };

    let patch = if let Some(patch) = captures.get(2) {
        Some(format!("{}.{}", season, patch.as_str()).parse().ok()?)
    } else {
        None
    };
    
    log::debug!("Client: {} {} {}", season, patch.unwrap_or(0.00), netcl);

    Some(Build {
        season,
        patch,
        netcl,
    })
}

// pub async fn redirect(req: &HttpRequest) -> Result<impl Responder> {
//     let client = reqwest::Client::new();
    
//     let headers = {
//         let mut headers = reqwest::header::HeaderMap::new();
        
//         for (key, val) in req.headers() {
//             headers.insert(key, val.clone());
//         }
        
//         headers
//     };
//     let path = CENTRAL.to_owned() + req.path();
    
//     log::debug!("Redirecting to {}", path);
    
//     let res = client.request(
//         req.method().clone(),
//         path
//     )
//     .headers(headers)
//     .send().await?;
    
//     Ok(
//         HttpResponse::build(res.status())
//         .body(res.bytes().await?)
//     )
// }

pub fn redirect(req: &HttpRequest) -> impl Responder {
    let path = CENTRAL.to_owned() + req.path();
    
    log::debug!("Redirect to {}", path);
    
    HttpResponse::TemporaryRedirect()
        .append_header(("Location", path))
        .finish()
}