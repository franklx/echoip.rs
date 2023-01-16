mod api;
mod error;
mod geoip_lookup;
mod guard;
mod model;
mod templates;

use actix_web::{
    http,
    middleware::{ErrorHandlers, Logger},
    route, web, App, HttpServer,
};
use actix_web_rust_embed_responder::{EmbedResponse, EmbedableFileResponse, IntoResponse};
use rust_embed_for_web::RustEmbed;

use log::debug;

use crate::{
    api::{html_response, json_response, plain_response, port_lookup},
    guard::AcceptHeader,
};

#[derive(RustEmbed)]
#[folder = "static/"]
struct Embed;

#[route("/{path:.*}", method = "GET", method = "HEAD")]
async fn static_embed(path: web::Path<String>) -> EmbedResponse<EmbedableFileResponse> {
    let path = if path.is_empty() { "index.html" } else { path.as_str() };
    Embed::get(path).into_response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "echoip=debug,actix_web=debug,info");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    debug!("Starting server.");

    debug!("Constructing the App");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, api::internal_server_error))
            .service(
                web::resource("/")
                    .route(web::get().guard(AcceptHeader { content_type: String::from("text/html") }).to(html_response))
                    .route(web::get().to(plain_response)),
            )
            .service(web::resource("/json").to(json_response))
            .service(web::resource("/port/{port}").to(port_lookup))
            .service(static_embed)
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
