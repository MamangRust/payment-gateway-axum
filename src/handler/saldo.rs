use axum::{
    extract::{Extension, Path, State}, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, post, put}, Json, Router
};
use serde_json::json;
use std::sync::Arc;
use crate::{
    middleware::jwt, domain::request::saldo::{CreateSaldoRequest, UpdateSaldoRequest}, state::AppState
};



pub async fn get_saldos(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.saldo_service.get_saldos().await {
        Ok(saldoes) => Ok((
            StatusCode::OK,
            Json(json!(saldoes))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}


pub async fn get_saldo(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.saldo_service.get_saldo(id).await {
        Ok(saldo) => Ok((
            StatusCode::OK,
            Json(json!(saldo))
        )),
       
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}

pub async fn get_saldo_users(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.saldo_service.get_saldo_users(id).await {
        Ok(saldo) => Ok((
            StatusCode::OK,
            Json(json!(saldo))
        )),
       
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}



pub async fn get_saldo_user(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.saldo_service.get_saldo_user(id).await {
        Ok(saldo) => Ok((
            StatusCode::OK,
            Json(json!(saldo))
        )),
       
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}


pub async fn create_saldo(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateSaldoRequest>,
  
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.saldo_service.create_saldo(&body).await {
        Ok(response) => Ok((
            StatusCode::CREATED,
            Json(json!(response))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}



pub async fn update_saldo(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(mut body): Json<UpdateSaldoRequest>,
   
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    body.saldo_id = id;
    
    match data.di_container.saldo_service.update_saldo(&body).await {
        Ok(response) => Ok((
            StatusCode::OK,
            Json(json!(response))
        )),
       
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}


pub async fn delete_saldo(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>, // JWT middleware check
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.saldo_service.delete_saldo(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Saldo deleted successfully"
            }))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}


pub fn saldos_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/saldos", get(get_saldos))
        .route("/api/saldos/:id", get(get_saldo))
        .route("/api/saldos/users/:id", get(get_saldo_users))
        .route("/api/saldos/user/:id", get(get_saldo_user))
        .route("/api/saldos", post(create_saldo))
        .route("/api/saldos/:id", put(update_saldo))
        .route("/api/saldos/:id", delete(delete_saldo))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth)).with_state(app_state.clone())


   
     
}