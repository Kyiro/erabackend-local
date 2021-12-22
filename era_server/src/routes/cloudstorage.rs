use actix_web::*;
use crate::{Result, State};
use crate::utils::{get_build, Build, redirect};

#[get("/api/cloudstorage/user/{id}")]
pub async fn user_list(
    app: web::Data<State>,
    req: HttpRequest
) -> Result<impl Responder> {
    let build = get_build(&req).unwrap_or(Build::default());
    Ok(HttpResponse::Ok().json(app.list_user_cloudstorage(build.season)?))
}

#[get("/api/cloudstorage/user/{id}/{file}")]
pub async fn user_get_file(
    app: web::Data<State>,
    req: HttpRequest,
    path: web::Path<(String, String)>
) -> Result<impl Responder> {
    let (_, file) = path.into_inner();
    let build = get_build(&req).unwrap_or(Build::default());
    
    Ok(
        HttpResponse::Ok()
        .insert_header(("Content-Type", "application/octet-stream"))
        .body(app.get_user_file(build.season, file)?)
    )
}

#[put("/api/cloudstorage/user/{id}/{file}")]
pub async fn user_put_file(
    app: web::Data<State>,
    body: web::Bytes,
    req: HttpRequest,
    path: web::Path<(String, String)>
) -> Result<impl Responder> {
    let (_, file) = path.into_inner();
    let build = get_build(&req).unwrap_or(Build::default());
    
    app.save_user_file(build.season, file, body.to_vec())?;
    
    Ok(HttpResponse::Ok())
}

#[get("/api/cloudstorage/system")]
pub async fn system_list(req: HttpRequest) -> impl Responder {
    redirect(&req)
}

#[get("/api/cloudstorage/system/config")]
pub async fn system_config() -> impl Responder {
    HttpResponse::NoContent().json(Vec::<i8>::new())
}

#[get("/api/cloudstorage/system/{file}")]
pub async fn system_get_file(req: HttpRequest) -> impl Responder {
    redirect(&req)
}