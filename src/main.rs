use std::net::Ipv4Addr;

use app::{AppState, db_conn, router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod infra;

macro_rules! expect_env {
    ($var_name:expr) => {
        std::env::var($var_name).expect(concat!("env missing: ", $var_name))
    };
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(
            "debug,hyper=off,rustls=error,tungstenite=error,hickory=off",
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_line_number(true)
                .with_file(true),
        )
        .init();

    dotenvy::dotenv().ok();

    let db = db_conn(&expect_env!("MONGODB_URI")).await;

    let state = AppState::new(db, expect_env!("OPEN_ROUTE_API_KEY"));

    let cors = tower_http::cors::CorsLayer::permissive();

    let app = router(state).layer(cors);

    let address = (Ipv4Addr::UNSPECIFIED, 8080);
    tracing::debug!("Server running on {:?}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind to network address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server")
}
