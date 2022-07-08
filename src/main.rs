use anyhow::Result;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use std::env;
use tracing::{info, warn};

use uptimebeaver_gateway::api::health::Health;
use utils::database::{DataBase, DB};

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv()?;

    utils::logging::setup(env::var("RUST_LOG")?)?;

    info!("starting service.");

    let port = env::var("PORT")?;
    let addr = format!("127.0.0.1:{}", port);
    let oas_addr = format!("http://localhost:{}/api", port);

    let api_service = OpenApiService::new((Health), "UptimeBeaver API", "1.0").server(&oas_addr);
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind(addr.as_str())).run(Route::new().nest("/api", api_service).nest("/", ui)).await?;

    warn!("quitting service");

    Ok(())
}
