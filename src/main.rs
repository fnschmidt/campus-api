use std::env;

use constants::{
    CD_CERT_PEM, LOGIN_RATELIMIT_QUOTA, LOGIN_RATELIMIT_RESTORE_INTERVAL_SEC, RATELIMIT_QUOTA,
    RATELIMIT_RESTORE_INTERVAL_SEC, set_statics_from_env,
};
use tokio::net::TcpListener;

mod auth;
pub mod campus_backend;
mod color_stuff;
mod constants;
mod encryption;
mod ratelimit_keyextractor;
mod routes;
mod services;
mod types;

#[tokio::main]
async fn main() {
    if env::var(pretty_env_logger::env_logger::DEFAULT_FILTER_ENV).is_err() {
        pretty_env_logger::formatted_timed_builder()
            .filter_level(log::LevelFilter::Info)
            .init();
    } else {
        pretty_env_logger::init_timed();
    }

    set_statics_from_env();

    log::info!("Starting Campus API...");
    log::info!("Rate limit: {}", RATELIMIT_QUOTA.get().unwrap());
    log::info!(
        "RL restore interval: every {} seconds",
        RATELIMIT_RESTORE_INTERVAL_SEC.get().unwrap()
    );
    log::info!("Login rate limit: {}", LOGIN_RATELIMIT_QUOTA.get().unwrap());
    log::info!(
        "Login RL restore interval: every {} seconds",
        LOGIN_RATELIMIT_RESTORE_INTERVAL_SEC.get().unwrap()
    );

    let buf = include_bytes!("GEANT TLS RSA 1.pem");
    let cert = reqwest::Certificate::from_pem(buf).unwrap();
    CD_CERT_PEM.set(cert).unwrap();

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Unable to start the server");

    log::info!("Listening on {}", listener.local_addr().unwrap());

    let app = routes::app().await;

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
