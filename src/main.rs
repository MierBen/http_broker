use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;

use std::collections::HashMap;
use std::sync::Mutex;
use std::{env, io};

mod config;
mod hanlders;

use config::SERVER_ADDR;
use hanlders::{get_handle, put_handle, Queue};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv().ok();

    // Create HashMap for Queue
    let queue: Queue = HashMap::new();
    let data = web::Data::new(Mutex::new(queue));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .service(get_handle)
            .service(put_handle)
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}
