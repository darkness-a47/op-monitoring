use actix_web::{HttpServer, App};
use web::web as router;
use gpio::gpio::init_pins;

pub mod web;
pub mod gpio;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_pins();
    println!("server is starting...");

    HttpServer::new(|| {
        App::new().service(router::handler)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
