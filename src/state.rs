use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    abstract_trait::{hashing::DynHashing, jwt::DynJwtService},
    config::{hashing::Hashing, jwt_config::JwtConfig},
    utils::di::DependenciesInject,
};

#[derive(Clone)]
pub struct AppState {
    pub di_container: DependenciesInject,
    pub jwt_config: DynJwtService,
}

impl AppState {
    pub fn new(pool: DatabaseConnection, jwt_secret: &str) -> Self {
        let jwt_config = Arc::new(JwtConfig::new(jwt_secret)) as DynJwtService;
        let hashing = Arc::new(Hashing::new()) as DynHashing;

        let di_container = DependenciesInject::new(pool, hashing, jwt_config.clone());

        Self {
            di_container,
            jwt_config,
        }
    }
}
