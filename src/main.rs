use actix_web::{middleware, web, App, HttpServer};

use async_std::sync::Mutex;
use std::collections::HashMap;
use std::{env, io};

mod config;
mod handlers;
mod models;

use config::SERVER_ADDR;
use handlers::{get_handle, put_handle};
use models::Queue;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let data = web::Data::new(Mutex::new(HashMap::new() as Queue));

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
