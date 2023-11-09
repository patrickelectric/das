use actix_web::{HttpRequest, HttpResponse};
use paperclip::actix::{api_v2_operation, web, web::Scope};

pub fn register_endpoints(scope: Scope) -> Scope {
    scope
        .route("/", web::get().to(root))
        .route("/list", web::get().to(list))
}

#[api_v2_operation]
pub fn root(_req: HttpRequest) -> HttpResponse {
    todo!()
}

#[api_v2_operation]
pub fn list(_req: HttpRequest) -> HttpResponse {
    todo!()
}
