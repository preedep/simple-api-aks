use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use log::{debug, info};
use opentelemetry::global;
use pretty_env_logger;
use rand::Rng;
use std::iter;
use tracing::trace;
use tracing_actix_web::TracingLogger;
use tracing_attributes::instrument;

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
#[instrument]
async fn manual_hello() -> impl Responder {
    trace!("calling manual_hello");
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
#[instrument]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    info!("Starting server");

    if let Ok(connection_string) = std::env::var("APPLICATIONINSIGHTS_CONNECTION_STRING") {
        debug!(
            "Starting application insights with connection string: {}",
            connection_string
        );

        let _tracer = opentelemetry_application_insights::new_pipeline_from_connection_string(
            connection_string,
        )
        .expect("valid connection string")
        .with_client(reqwest::Client::new())
        .with_live_metrics(true)
        .with_service_name("Simple API")
        .install_batch(opentelemetry_sdk::runtime::Tokio);
    }

    let rs = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new(
                r#"%a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.1")))
            .wrap(RequestTracing::new())
            .wrap(TracingLogger::default())
            .route("/hey", web::get().to(manual_hello))
            .route("/health", web::get().to(health_check))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await;

    global::shutdown_tracer_provider();
    global::shutdown_logger_provider();
    global::shutdown_meter_provider();

    rs
}
