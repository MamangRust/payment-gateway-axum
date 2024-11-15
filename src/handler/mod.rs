mod auth;
mod saldo;
mod topup;
mod transfer;
mod withdraw;
mod user;

use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;

use crate::state::AppState;

pub use self::auth::auth_routes;
pub use self::user::users_routes;
pub use self::saldo::saldos_routes;
pub use self::topup::topup_routes;
pub use self::transfer::transfers_routes;
pub use self::withdraw::withdraw_routes;


pub struct AppRouter;

impl AppRouter {
    pub async fn serve(port: u16, app_state: AppState) -> Result<(), Box<dyn std::error::Error>> {
       
        let shared_state = Arc::new(app_state);

        
        let router = Router::new()
            .merge(auth_routes(shared_state.clone()))
            .merge(users_routes(shared_state.clone()))
            .merge(saldos_routes(shared_state.clone()))
            .merge(topup_routes(shared_state.clone()))
            .merge(transfers_routes(shared_state.clone()))
            .merge(withdraw_routes(shared_state.clone()));
            

       
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(addr).await?;
        println!("Server running on http://{}", listener.local_addr()?);

        axum::serve(listener, router).await.unwrap();
        Ok(())
    }
}