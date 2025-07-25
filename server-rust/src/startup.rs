use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, get, http, web};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{
    create_question::create_question, create_room::create_room,
    get_room_questions::get_room_questions, get_rooms::get_rooms,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(hello)
            .service(create_room)
            .service(create_question)
            .service(get_rooms)
            .service(get_room_questions)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
