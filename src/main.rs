#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

use crate::api::{auth::AuthApi, user::UserApi};
use common::CONTEXT;
use log::info;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;

mod api;
mod common;
mod config;
mod middleware;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    // 初始化日志
    config::log::init_log();
    info!(
        " - Local: http://{}",
        CONTEXT
            .app_config
            .server
            .server_url
            .replace("0.0.0.0", "127.0.0.1")
    );

    let api_service = OpenApiService::new((AuthApi, UserApi), "poem web", "0.1.0")
        .server("http://172.18.3.1:3300");
    let ui = api_service.swagger_ui();

    let listener = TcpListener::bind(&CONTEXT.app_config.server.server_url);
    Server::new(listener)
        .run(
            Route::new()
                .nest("/", api_service)
                .nest("/swagger", ui)
                .with(middleware::new_cors()),
        )
        .await
        .unwrap();
}
