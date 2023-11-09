use actix_web::{HttpRequest, HttpResponse};
use paperclip::actix::{api_v2_operation, web, web::Scope};

pub fn register_endpoints(scope: Scope) -> Scope {
    scope.route("/id", web::get().to(root))
}

#[api_v2_operation]
pub fn root(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Potato")
}
