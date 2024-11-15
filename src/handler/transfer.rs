use crate::{
    domain::request::transfer::{CreateTransferRequest, UpdateTransferRequest},
    middleware::jwt,
    state::AppState,
};
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

pub async fn get_transfers(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.transfer_service.get_transfers().await {
        Ok(response) => Ok((StatusCode::OK, Json(json!(response)))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

pub async fn get_transfer(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.transfer_service.get_transfer(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(json!(response)))),

        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

pub async fn get_transfer_users(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data
        .di_container
        .transfer_service
        .get_transfer_users(id)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(json!(response)))),

        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

pub async fn get_transfer_user(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data
        .di_container
        .transfer_service
        .get_transfer_user(id)
        .await
    {
        Ok(saldo) => Ok((StatusCode::OK, Json(json!(saldo)))),

        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

pub async fn create_transfer(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTransferRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data
        .di_container
        .transfer_service
        .create_transfer(&body)
        .await
    {
        Ok(response) => Ok((StatusCode::CREATED, Json(json!(response)))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

pub async fn update_transfer(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(mut body): Json<UpdateTransferRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    body.transfer_id = id;

    match data
        .di_container
        .transfer_service
        .update_transfer(&body)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(json!(response)))),

        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

pub async fn delete_transfer(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.topup_service.delete_topup(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Transfer deleted successfully"
            })),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

pub fn transfers_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/transfer", get(get_transfers))
        .route("/api/transfer/:id", get(get_transfer))
        .route("/api/transfer/users/:id", get(get_transfer_users))
        .route("/api/transfer/user/:id", get(get_transfer_user))
        .route("/api/transfer", post(create_transfer))
        .route("/api/transfer/:id", put(update_transfer))
        .route("/api/transfer/:id", delete(delete_transfer))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
        .with_state(app_state.clone())
}
