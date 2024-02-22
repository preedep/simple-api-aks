use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info;
use pretty_env_logger;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    info!("Starting server");
    HttpServer::new(|| App::new().route("/hey", web::get().to(manual_hello)))
        .bind(("0.0.0.0", 8888))?
        .run()
        .await
}
