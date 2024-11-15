use dotenv::dotenv;

use example_payment_gateway_axum::config::{config::Config, database::ConnectionManager};
use example_payment_gateway_axum::handler::AppRouter;
use example_payment_gateway_axum::migrations::m20220101_000001_create_table::Migration;
use example_payment_gateway_axum::state::AppState;
use example_payment_gateway_axum::utils::log_tracing;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    log_tracing::tracing();

    let config = Config::init();

    let db_pool =
        ConnectionManager::new_pool::<Migration>(&config.database_url, config.run_migrations)
            .await?;

    let port = config.port;

    let state = AppState::new(db_pool, &config.jwt_secret);

    println!("🚀 Server started successfully");

    AppRouter::serve(port, state).await
}
