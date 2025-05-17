use std::net::Ipv4Addr;

use app::{AppState, db_conn, router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod infra;

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

    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("FATAL Error: {:?}", panic_info);
    }));

    dotenvy::dotenv().ok();

    let db = db_conn().await;

    let state = AppState::new(db);

    let app = router(state);

    let address = (Ipv4Addr::UNSPECIFIED, 8080);
    tracing::debug!("Server running on {:?}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind to network address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server")
}
