use std::net::TcpListener;

use actix_web::{HttpResponse, Responder, get};

use crate::{db::configuration::get_configuration, env::get_env_var};

pub mod db;
pub mod env;
pub mod routes;
pub mod startup;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = get_env_var("PORT").parse::<u16>().unwrap();
    let connection = get_configuration().await.unwrap();
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener, connection)?.await
}
