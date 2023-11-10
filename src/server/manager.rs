use actix_web::{error::JsonPayloadError, App, HttpRequest, HttpServer};
use paperclip::{
    actix::{web, OpenApiExt},
    v2::models::{Api, Info},
};
use actix_web::HttpResponse;

use super::{pages, plugins, vehicle};
use paperclip::actix::api_v2_operation;

fn json_error_handler(error: JsonPayloadError, _: &HttpRequest) -> actix_web::Error {
    println!("Problem with json: {error}");
    error.into()
}

fn construct_dynamic_openapi_spec() -> serde_json::Value {
    // Construct the OpenAPI spec for your dynamic endpoints manually
    // This is a simplified example, you would need to construct a valid OpenAPI spec JSON
    serde_json::json!({
        "paths": {
            "/dynamic_endpoint": {
                "get": {
                    "summary": "Dynamically generated endpoint",
                    "responses": {
                        "200": {
                            "description": "successful operation"
                        }
                    }
                }
            }
        }
    })
}

#[api_v2_operation]
fn openapi_json() -> HttpResponse {
    let spec = construct_dynamic_openapi_spec();
    HttpResponse::Ok().json(spec)
}


// Start REST API server with the desired address
pub async fn run(server_address: &str) -> Result<(), std::io::Error> {
    let server_address = server_address.to_string();

    HttpServer::new(move || {
        let vehicle_service = vehicle::register_endpoints(web::scope("/vehicle"));
        // TODO: Do not use vehicle, dah
        let plugins_service = plugins::register_endpoints(web::scope("/plugins"));

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap_api_with_spec(Api {
                info: Info {
                    version: format!(
                        "{}-{} ({})",
                        env!("CARGO_PKG_VERSION"),
                        env!("VERGEN_GIT_SHA"),
                        env!("VERGEN_BUILD_DATE")
                    ),
                    title: env!("CARGO_PKG_NAME").to_string(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_json_spec_at("/docs.json")
            .with_swagger_ui_at("/docs")
            .service(web::resource("/api/openapi.json").route(web::get().to(openapi_json)))
            // Record services and routes for paperclip OpenAPI plugin for Actix.
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
            .route("/", web::get().to(pages::root))
            .service(vehicle_service)
            .service(plugins_service)
            .build()
    })
    .bind(server_address)
    .expect("Failed starting web API")
    .run()
    .await
}
