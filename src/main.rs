mod types;
mod server;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    println!("Hello, world!");
    server::manager::run("0.0.0.0:8080").await
}
