use anyhow::Result;
use std::env;
use tonic::transport::Server;
use tracing::{info, warn};

use service_template::services::greeter::GreeterService; // TODO: Change me
use utils::database::{DataBase, DB};

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv()?;

    utils::logging::setup(env::var("RUST_LOG")?)?;

    info!("setting up database");

    let db = DataBase::from_uri(env::var("DATABASE_URL")?).await?;
    DB.set(db).expect("unable to set DB");

    info!("finished setting up database");

    info!("starting service.");

    let port = env::var("PORT")?;
    let addr = format!("[::]:{}", port).parse()?;

    Server::builder()
        .add_service(GreeterService::create_server()) // TODO: Change me
        .serve(addr)
        .await?;

    warn!("quitting service");

    Ok(())
}
