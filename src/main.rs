use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware};
use log::info;
use pretty_env_logger;
use rand::Rng;
use std::iter;
use actix_web::middleware::Logger;
use opentelemetry_sdk::trace::Tracer;

#[derive(serde::Deserialize, serde::Serialize)]
struct Payload {
    first_name: String,
    last_name: String,
    address: String,
    age: u8,
}
#[derive(serde::Deserialize, serde::Serialize)]
struct PayloadList {
    payload: Vec<Payload>,
}

fn generate(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}

async fn manual_hello() -> impl Responder {
    let mut payload_list: Vec<Payload> = Vec::new();
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(1..10000);

    for _ in 0..num {
        let payload = Payload {
            first_name: generate(30),
            last_name: generate(30),
            address: generate(100),
            age: 42,
        };
        payload_list.push(payload);
    }
    let payload = PayloadList {
        payload: payload_list,
    };
    HttpResponse::Ok().json(payload)
}
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    info!("Starting server");

    if let Ok(connection_string) = std::env::var("APPLICATIONINSIGHTS_CONNECTION_STRING"){
        let _tracer = opentelemetry_application_insights::new_pipeline_from_connection_string(connection_string)
            .expect("valid connection string")
            .with_client(reqwest::Client::new())
            .with_live_metrics(true)
            .install_batch(opentelemetry_sdk::runtime::Tokio);
    }

    HttpServer::new(||
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.1")))
            .route("/hey", web::get().to(manual_hello))
            .route("/health", web::get().to(health_check))
        )
        .bind(("0.0.0.0", 8888))?
        .run()
        .await

   // global::shutdown_tracer_provider();

}
