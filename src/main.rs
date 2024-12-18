use anyhow::Result;
use api::post::post_mod::post_api_server::PostApiServer;
use api::Api;
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::env;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let addr = env::var("ADDR").expect("ADDR must be set").parse()?;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = Database::connect(&database_url).await?;
    Migrator::up(&connection, None).await?;

    Server::builder()
        .add_service(PostApiServer::with_interceptor(
            Api { connection },
            auth::jwt_interceptor,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
