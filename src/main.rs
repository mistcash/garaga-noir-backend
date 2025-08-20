use garaga_noir_backend::app;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing

    info!(
        "Starting Cairo Compilation API v{}",
        env!("CARGO_PKG_VERSION")
    );

    // Start the server
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    let addr = format!("0.0.0.0:{}", port);
    println!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app()).await?;

    Ok(())
}
