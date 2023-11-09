mod types;
mod server;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    server::manager::run("0.0.0.0:8080").await
}
