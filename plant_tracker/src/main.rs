use axum::{Router, routing::get};
use plant_tracker::bot;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    // Local development:
    // run the bot directly without starting the HTTP server.
    tracing::info!("Bot has started...");
    bot::plant_bot().await;

    // Deployment:
    // run the bot in a separate task while Axum keeps the HTTP server alive.
    // tokio::spawn(async {
    //     tracing::info!("Bot has started...");
    //     bot::plant_bot().await;
    // });

    // let app = Router::new().route("/", get(|| async { "OK" }));

    // // Render provides the PORT environment variable.
    // let port: u16 = std::env::var("PORT")
    //     .unwrap_or_else(|_| "10000".to_string())
    //     .parse()
    //     .unwrap();

    // let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
    //     .await
    //     .unwrap();

    // axum::serve(listener, app).await.unwrap();
}