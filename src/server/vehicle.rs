use actix_web::{HttpRequest, HttpResponse};
use paperclip::actix::{api_v2_operation, web, web::Scope};
use super::logs;

pub fn register_endpoints(scope: Scope) -> Scope {
    let log_service = logs::register_endpoints(web::scope("/logs"));

    scope
        .service(log_service)
        .route("/", web::get().to(root))
        .route("/id", web::get().to(id))
        // TODO: Break arm to receive parameters
        .route("/arm", web::post().to(arm))
        .route("/disarm", web::post().to(disarm))
        .route("/is_armed", web::get().to(is_armed))
        .route("/altitude", web::get().to(altitude))
        .route("/attitude", web::get().to(attitude))
        .route("/batteries", web::get().to(batteries))
        .route("/cpu_load", web::get().to(cpu_load))
        .route("/mode", web::get().to(mode))
        .route("/modes_available", web::get().to(modes_available))
        .route("/position", web::get().to(position))
        .route("/velocity", web::get().to(velocity))
        .route("/power_supply", web::get().to(power_supply))
        .route("/parameters", web::get().to(parameters))
        .route("/status", web::get().to(status))
}

#[api_v2_operation]
pub fn root(_req: HttpRequest) -> HttpResponse {
    todo!()
}

#[api_v2_operation]
pub fn id(_req: HttpRequest) -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn arm() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn disarm() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn is_armed() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn altitude() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn attitude() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn batteries() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn cpu_load() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn mode() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn modes_available() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn position() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn velocity() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn power_supply() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn parameters() -> HttpResponse {
    todo!()
}

#[api_v2_operation]
async fn status() -> HttpResponse {
    todo!()
}